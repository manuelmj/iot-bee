use std::collections::HashMap;
use std::sync::Mutex;
use async_trait::async_trait;
use serde_json::Value;
use super::schemas::{FieldSchema, ValidationRule};
use super::compiler::Program;
use super::vm::{Vm, VmError};
use domain::error::{IoTBeeError, DomainValidationError};
use domain::entities::data_consumer_types::DataConsumerRawType;
use domain::outbound::data_processor_actions::DataProcessorActions;

// Error interno del procesador (distinto de domain::error::PipelineError)
#[derive(Debug)]
pub enum ProcessorError {
    RequiredFieldMissing(String),
    ValidationFailed { field: String, reason: String },
    ExecutionError { field: String, error: VmError },
}

impl From<ProcessorError> for IoTBeeError {
    fn from(e: ProcessorError) -> Self {
        let domain_err = match e {
            ProcessorError::RequiredFieldMissing(field) => {
                DomainValidationError::MissingField { field_name: field }
            }
            ProcessorError::ValidationFailed { field, reason } => {
                DomainValidationError::InvalidFieldValue { field_name: field, reason }
            }
            ProcessorError::ExecutionError { field, error } => {
                DomainValidationError::DataFormatError {
                    reason: format!("Error al ejecutar operación en campo '{}': {:?}", field, error),
                }
            }
        };
        IoTBeeError::DomainValidationError(domain_err)
    }
}

// La versión compilada de un campo:
// el AST ya no existe, solo el bytecode listo para ejecutar
struct CompiledField {
    required: bool,
    default: Option<f64>,
    validation: Option<ValidationRule>,
    // None = no hay operación, pasar valor directo
    program: Option<Program>,
}

// Mutex<Vm> permite interior mutability: process() puede tomar &self
// en lugar de &mut self, lo que permite implementar el trait DataProcessorActions.
pub struct PipelineDataProcessor {
    fields: HashMap<String, CompiledField>,
    vm: Mutex<Vm>,
}

impl PipelineDataProcessor {
    // Construye el pipeline compilando todas las operaciones a partir del JSON del schema.
    // Este método se llama UNA sola vez. A partir de aquí solo se necesita el dato crudo.
    // schema_json: JSON con el formato { "campo": { "type": ..., "required": ..., "operation": ... } }
    pub fn new(schema_json: &str) -> Result<Self, IoTBeeError> {
        let field_defs: HashMap<String, FieldSchema> = serde_json::from_str(schema_json)
            .map_err(|e| DomainValidationError::DataFormatError {
                reason: format!("Schema inválido: {}", e),
            })?;

        let fields = field_defs
            .into_iter()
            .map(|(name, field)| {
                let program = field.operation
                    .as_ref()
                    .map(|expr| Program::compile(expr));

                let compiled = CompiledField {
                    required: field.required,
                    default: field.default,
                    validation: field.validation,
                    program,
                };
                (name, compiled)
            })
            .collect();

        Ok(PipelineDataProcessor {
            fields,
            vm: Mutex::new(Vm::new()),
        })
    }

    // Procesa un registro. Se llama miles de veces.
    // `record` es el JSON plano: {"temperatura": 20.0, ...}
    pub fn process(
        &self,
        record: &HashMap<String, f64>,
    ) -> Result<HashMap<String, f64>, ProcessorError> {
        let mut vm = self.vm.lock().unwrap();
        let mut output = HashMap::new();

        for (field_name, compiled) in &self.fields {
            // 1. Resolver el valor crudo
            let raw = match record.get(field_name) {
                Some(&v) => v,
                None => match compiled.default {
                    Some(d) => d,
                    None if compiled.required => {
                        return Err(ProcessorError::RequiredFieldMissing(
                            field_name.clone()
                        ));
                    }
                    None => continue, // campo opcional ausente: omitir
                },
            };

            // 2. Validar contra min/max
            if let Some(rule) = &compiled.validation {
                if let Some(min) = rule.min {
                    if raw < min {
                        return Err(ProcessorError::ValidationFailed {
                            field: field_name.clone(),
                            reason: format!("{} < min({})", raw, min),
                        });
                    }
                }
                if let Some(max) = rule.max {
                    if raw > max {
                        return Err(ProcessorError::ValidationFailed {
                            field: field_name.clone(),
                            reason: format!("{} > max({})", raw, max),
                        });
                    }
                }
            }

            // 3. Ejecutar la operación (o pasar directo)
            let result = match &compiled.program {
                Some(prog) => {
                    vm.run(prog, record)
                        .map_err(|e| ProcessorError::ExecutionError {
                            field: field_name.clone(),
                            error: e,
                        })?
                }
                None => raw, // sin operación: el valor pasa tal cual
            };

            output.insert(field_name.clone(), result);
        }

        Ok(output)
    }
}

// Convierte un JSON string a HashMap<String, f64>.
// Soporta Number, Bool (true→1.0, false→0.0). Falla para otros tipos.
fn parse_record(json: &str) -> Result<HashMap<String, f64>, IoTBeeError> {
    let raw: HashMap<String, Value> = serde_json::from_str(json)
        .map_err(|e| DomainValidationError::DataFormatError {
            reason: format!("JSON de datos inválido: {}", e),
        })?;

    let mut record = HashMap::new();
    for (key, val) in raw {
        let num: f64 = match val {
            Value::Number(n) => n.as_f64().ok_or_else(|| DomainValidationError::DataFormatError {
                reason: format!("Campo '{}' no puede convertirse a f64", key),
            })?,
            Value::Bool(b) => if b { 1.0 } else { 0.0 },
            other => {
                return Err(DomainValidationError::DataFormatError {
                    reason: format!("Campo '{}' tiene tipo no soportado: {:?}", key, other),
                }.into())
            }
        };
        record.insert(key, num);
    }
    Ok(record)
}

// Deserializa el JSON del schema almacenado en PipelineValidationSchemaModel
// en el tipo PipelineSchema que el compilador entiende.
#[async_trait]
impl DataProcessorActions for PipelineDataProcessor {
    async fn process_data(
        &self,
        data_to_process: &DataConsumerRawType,
    ) -> Result<DataConsumerRawType, IoTBeeError> {
        // 1. Parsear el payload crudo a un mapa numérico
        let record = parse_record(data_to_process.value())?;

        // 2. Procesar con el schema ya compilado: aplica operaciones y validaciones
        let output = self.process(&record)?;

        // 3. Serializar el resultado de vuelta a JSON y envolverlo en DataConsumerRawType
        let json = serde_json::to_string(&output)
            .map_err(|e| DomainValidationError::DataFormatError {
                reason: format!("Error al serializar resultado: {}", e),
            })?;

        DataConsumerRawType::new(json)
    }
}

// fn main() {
//     // Este string vendría de tu BD en producción
//     let schema_json = r#"
//     {
//       "version": 1,
//       "fields": {
//         "temperatura": {
//           "type": "float",
//           "required": true,
//           "default": null,
//           "validation": { "min": -50.0, "max": 150.0 },
//           "operation": {
//             "type": "bin_op",
//             "op": "mul",
//             "left":  { "type": "var", "name": "temperatura" },
//             "right": { "type": "num", "value": 1.8 }
//           }
//         },
//         "humedad": {
//           "type": "int",
//           "required": false,
//           "default": 0.0,
//           "validation": { "min": 0.0, "max": 100.0 },
//           "operation": null
//         }
//       }
//     }
//     "#;

//     // Deserializar y compilar — solo una vez
//     let schema: PipelineSchema = serde_json::from_str(schema_json)
//         .expect("schema inválido");
//     let mut pipeline = Pipeline::new(schema);

//     // Simular registros que llegan en producción
//     let registros: Vec<HashMap<String, f64>> = vec![
//         [("temperatura".into(), 20.0), ("humedad".into(), 65.0)].into(),
//         [("temperatura".into(), 35.5), ("humedad".into(), 80.0)].into(),
//         [("temperatura".into(), 0.0),  ("humedad".into(), 45.0)].into(),
//     ];

//     for registro in &registros {
//         match pipeline.process(registro) {
//             Ok(resultado) => {
//                 println!("entrada:  {:?}", registro);
//                 println!("salida:   {:?}", resultado);
//                 // temperatura 20.0 → 36.0  (×1.8)
//                 // humedad     65.0 → 65.0  (sin cambio)
//                 println!();
//             }
//             Err(e) => eprintln!("error: {:?}", e),
//         }
//     }
// }