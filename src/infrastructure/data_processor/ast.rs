use serde::{Deserialize, Serialize};

// Un nodo del árbol. Es un enum porque cada nodo
// puede ser una de varias cosas distintas.
//
// El atributo serde(tag = "type") le dice a serde que
// use el campo "type" del JSON para saber qué variante
// deserializar. Así:
//   { "type": "num", "value": 3.0 }  → Expr::Num
//   { "type": "var", "name": "x" }   → Expr::Var
//   { "type": "bin_op", ... }         → Expr::BinOp
//
// rename_all = "snake_case" convierte los nombres de
// variantes a minúsculas con guiones: BinOp → bin_op

#[derive(Debug,Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Expr {
    //un numero constanste {type : "num", value: 3.0}
    Num { value: f64 },
    // Una variable (campo del registro): { "type": "var", "name": "temperatura" }
    // En tiempo de ejecución, se reemplaza por el valor
    // real que llega en el JSON de producción.
    Var { name: String },
    // Una operación entre dos sub-expresiones.
    // Usa Box<Expr> porque Expr contiene Expr:
    // un enum no puede tener tamaño fijo si se contiene
    // a sí mismo directamente. Box pone el hijo en el heap,
    // rompiendo el ciclo de tamaño infinito
    BinOp { op: Op, left: Box<Expr>, right: Box<Expr> },
}

// los operadores soportados
#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}