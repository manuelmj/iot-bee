// use crat



use std::collections::HashMap;
use super::schemas::{PipelineSchema, ValidationRule};
use super::compiler::Program;
use super::vm::{Vm, VmError};

#[derive(Debug)]
pub enum PipelineError {
    RequiredFieldMissing(String),
    ValidationFailed { field: String, reason: String },
    ExecutionError { field: String, error: VmError },
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


pub struct PipelineDataProcessor {
    fields: HashMap<String, CompiledField>,
    vm: Vm,
}

// use crate::domain::outbound::data_processor_actions::DataProcessorActions;



impl PipelineDataProcessor {
    // Construye el pipeline compilando todas las operaciones.
    // Este método se llama UNA sola vez al cargar el schema.
    pub fn new(schema: PipelineSchema) -> Self {


        let fields = schema.fields
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

        PipelineDataProcessor {
            fields,
            vm: Vm::new(),
        }
    }

    // Procesa un registro. Se llama miles de veces.
    // `record` es el JSON plano: {"temperatura": 20.0, ...}
    pub fn process(
        &mut self,
        record: &HashMap<String, f64>,
    ) -> Result<HashMap<String, f64>, PipelineError> {
        let mut output = HashMap::new();

        for (field_name, compiled) in &self.fields {
            // 1. Resolver el valor crudo
            let raw = match record.get(field_name) {
                Some(&v) => v,
                None => match compiled.default {
                    Some(d) => d,
                    None if compiled.required => {
                        return Err(PipelineError::RequiredFieldMissing(
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
                        return Err(PipelineError::ValidationFailed {
                            field: field_name.clone(),
                            reason: format!("{} < min({})", raw, min),
                        });
                    }
                }
                if let Some(max) = rule.max {
                    if raw > max {
                        return Err(PipelineError::ValidationFailed {
                            field: field_name.clone(),
                            reason: format!("{} > max({})", raw, max),
                        });
                    }
                }
            }

            // 3. Ejecutar la operación (o pasar directo)
            let result = match &compiled.program {
                Some(prog) => {
                    self.vm.run(prog, record)
                        .map_err(|e| PipelineError::ExecutionError {
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




// mod ast;
// mod schema;
// mod compiler;
// mod vm;
// mod pipeline;

// use std::collections::HashMap;
// use pipeline::Pipeline;
// use schema::PipelineSchema;

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