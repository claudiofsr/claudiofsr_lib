use regex::Regex;
use std::{fmt::Display, sync::LazyLock};

/*
How to construct large const arrays in Rust without manually writing values to construct it
https://stackoverflow.com/questions/74320587/how-to-construct-large-const-arrays-in-rust-without-manually-writing-values-to-c
let all_cst: Vec<u16> = (1..=99).collect();
let all_cst: [u16; 99] = core::array::from_fn(|i| (i + 1) as u16);

Note that arrays in Rust are different from vectors.

An array is a fixed length stack-based set of things.
let x = [1, 2, 3, 4] is an array.
While you can read from it and mutate individual elements, you cannot grow it.

A vector is heap based and has variable length (and is growable).
let y = vec![1, 2, 3, 4] is a vector.
You can append elements to this if you need.
*/

/// convert `array: [T; N]` to `Vec<Option<U>>`
///
/// Example:
/// ```
/// use claudiofsr_lib::array_to_vec;
///
/// let array = [1, 2, 3];
/// let vector_opt: Vec<Option<u16>> = array_to_vec(array);
///
/// // Use `flatten()` to convert vector of Options or
/// // Results to only the successful values:
/// let vector: Vec<u16> = vector_opt
///     .into_iter()
///     .flatten()
///     .collect();
///
/// assert_eq!(vector, [1, 2, 3]);
/// ```
///
/// <https://practice.rs/generics-traits/const-generics.html>
///
/// <https://doc.rust-lang.org/reference/items/generics.html>
pub fn array_to_vec<T, U, const N: usize>(array: [T; N]) -> Vec<U>
where
    U: std::convert::From<T>,
{
    array.into_iter().map(T::into).collect()
}

/**
Generic numeric conversion.

Try to convert `&[T]` to `Vec<U>`.

Example:
```
    use claudiofsr_lib::try_convert;

    let array: [i16; 5] = [20, 35, 456, -15, 7];
    let result1: Vec<f32> = try_convert(&array);

    let vector1: Vec<i16> = vec![20, 35, 456, -15, 7];
    let result2: Vec<f64> = try_convert(&vector1);

    let vector2: Vec<i16> = vec![20, 35, 456, 7];
    let result3: Vec<u64> = try_convert(&vector2);

    let vector3: Vec<i64> = vec![20, 35, 456, 7];
    let result4: Vec<u16> = try_convert(&vector3);

    let valid1: Vec<f32> = vec![20.0, 35.0, 456.0, -15.0, 7.0];
    let valid2: Vec<f64> = vec![20.0, 35.0, 456.0, -15.0, 7.0];
    let valid3: Vec<u64> = vec![20, 35, 456, 7];
    let valid4: Vec<u16> = vec![20, 35, 456, 7];

    assert_eq!(valid1, result1);
    assert_eq!(valid2, result2);
    assert_eq!(valid3, result3);
    assert_eq!(valid4, result4);
```
<https://users.rust-lang.org/t/generic-numeric-conversion/37052>

<https://www.justanotherdot.com/posts/how-do-you-cast-generic-values-youre-sure-are-numbers>
*/
pub fn try_convert<T, U>(slice: &[T]) -> Vec<U>
where
    T: Copy,
    U: TryFrom<T>,
    <U as TryFrom<T>>::Error: Display,
{
    slice
        .iter()
        .map(|&type_t| match U::try_from(type_t) {
            Ok(type_u) => type_u,
            Err(why) => {
                let t = std::any::type_name::<T>();
                let u = std::any::type_name::<U>();
                panic!("Error converting from {t} to {u}: {why}")
            }
        })
        .collect()
}

/*
// todo!()
pub trait SliceExtension {
    fn try_convert2<U>(&self) -> Vec<U>;
}

impl<I: IntoIterator<Item=T>, T: Deref> SliceExtension for I {
    fn try_convert2<U>(&self) -> Vec<U>
    where
        U: TryFrom<T>,
        <U as TryFrom<T>>::Error: std::fmt::Display
    {
        let input: Vec<T> = self.into_iter().collect();
        let output: Vec<U> = try_convert(&input);
        output
    }
}
*/

/// Valores de 1 a 99
pub const CST_ALL: [u16; 99] = {
    let mut output = [0; 99];
    let mut index: usize = 0;

    while index < output.len() {
        output[index] = index as u16 + 1;
        index += 1;
    }

    output
};

/// Valores de 101 a 199
pub const BASE_CALC_SOMA: [u16; 99] = {
    let mut output = [0; 99];
    let mut index: usize = 0;

    while index < output.len() {
        output[index] = (index + 101) as u16;
        index += 1;
    }

    output
};

pub const CST_CREDITO: [u16; 14] = [50, 51, 52, 53, 54, 55, 56, 60, 61, 62, 63, 64, 65, 66];

pub const CST_CREDITO_BASICO: [u16; 7] = [50, 51, 52, 53, 54, 55, 56];

pub const CST_CREDITO_PRESUMIDO: [u16; 7] = [60, 61, 62, 63, 64, 65, 66];

/// Valores de 1 a 9 e 49
pub const CST_RECEITA_BRUTA: [u16; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 49];

pub const CSTS_NAO_TRIBUTADOS: [u16; 6] = [4, 6, 7, 8, 9, 49];

/// Valores de 1 a 18
pub const CODIGO_DA_NATUREZA_BC: [u16; 18] = {
    // Initialize an array of size 18 with all elements set to 0.
    let mut output = [0; 18];

    // Initialize an index variable starting at 0.
    let mut index: usize = 0;

    // Loop through the array and assign values from 1 to 18.
    while index < output.len() {
        // Assign the current index + 1 to the corresponding position in the array.
        output[index] = (index + 1) as u16;

        // Increment the index by 1.
        index += 1;
    }

    // Return the populated array.
    output
};

pub const CFOP_VENDA_DE_IMOBILIZADO: [u16; 3] = [5551, 6551, 7551];

/**
CFOP de Exportação:
```ignore
Grupo 7:

    7000 .. 7999,

Fim específico de exportação:

    1500, 1501, 1503, 1504, 1505, 1506,

    2500, 2501, 2503, 2504, 2505, 2506,

    3500, 3503,

    5500, 5501, 5502, 5503, 5504, 5505,

    6500, 6501, 6502, 6503, 6504, 6505,
```
### Fonte:

<https://www.gov.br/receitafederal/pt-br/assuntos/aduana-e-comercio-exterior/manuais/exportacao-portal-unico/situacoes-especiais-na-exportacao/exportacao-indireta>

<www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/cfop_cvsn_70_vigente>
*/
pub const CFOP_DE_EXPORTACAO: [u16; 1026] = {
    let mut output = [0; 1026];

    let fim_de_exportacao: [u16; 26] = [
        1500, 1501, 1503, 1504, 1505, 1506, 2500, 2501, 2503, 2504, 2505, 2506, 3500, 3503, 5500,
        5501, 5502, 5503, 5504, 5505, 6500, 6501, 6502, 6503, 6504, 6505,
    ];

    let mut index: usize = 0;

    while index < output.len() {
        if index < 26 {
            output[index] = fim_de_exportacao[index];
        } else {
            output[index] = (index - 26) as u16 + 7000;
        }

        index += 1;
    }

    output
};

// Regex, flags:
// x: verbose mode, ignores whitespace and allow line comments (starting with `#`)
// i: case-insensitive: letters match both upper and lower case
pub const PATTERN: &str = r#"(?xi)
# Esta é uma lista com possíveis Receitas Não Operacionais (outras receitas)
# a depender das atividades que constituam objeto da empresa:
    Atualiz.*Monet|   # Atualização Monetária
    Vend.*Imobiliz|   # Venda de Imobilizado
    Var.*Camb|        # Variação Cambial
    Desc.*Financ|     # Descontos Financeiros
    Desc.*Obtido|     # Descontos Obtidos
    Desp.*N.*Oper|    # Despesas Não Operacionais
    Rec.*Financ|      # Receitas Financeiras
    Rec.*N.*Oper|     # Receitas Não Operacionais
    Outras?\s*Rec|    # Outras Receitas (mesmo que Receitas Não Operacionais)
    Outras?\s*Desp|   # Outras Despesas
    Juro|             # Juros sobre Capital Próprio
    Selic|            # Selic é a taxa básica de juros da economia
    Hedge
"#;

pub static OUTRAS_RECEITAS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(PATTERN).expect("OUTRAS_RECEITAS_REGEX regex inválida!"));

#[cfg(test)]
mod functions {
    use super::*;

    // cargo test -- --help
    // cargo test -- --nocapture
    // cargo test -- --show-output

    #[test]
    fn find_other_revenues() {
        // cargo test -- --show-output find_other_revenues
        let lines = vec![
            "foo bar",
            "Atualização Monetária",
            "Outras Receitas Operacionais",
            "Juros sobre Capital Próprio",
            "Descontos",
        ];

        let mut result = Vec::new();

        for line in lines {
            let find: bool = OUTRAS_RECEITAS_REGEX.is_match(line);
            result.push(find);
        }

        assert_eq!(vec![false, true, true, true, false], result);
    }

    #[test]
    fn get_cst_all_options() {
        // cargo test -- --show-output get_cst_all_options
        let csts: Vec<Option<u16>> = array_to_vec(CST_ALL);
        println!("csts: {csts:?}");

        assert_eq!(csts[0], Some(1));
        assert_eq!(csts[98], Some(99));
    }

    #[test]
    fn get_cfop_options() {
        // cargo test -- --show-output get_cfop_options

        // let cfops: Vec<Option<u16>> = array_to_vec(CFOP_DE_EXPORTACAO);
        let cfops = CFOP_DE_EXPORTACAO.map(Some);

        println!("cfops: {cfops:?}");

        assert_eq!(cfops[0], Some(1500));
        assert_eq!(cfops[26], Some(7000));
    }

    #[test]
    fn convert() {
        // cargo test -- --show-output convert
        let nats: Vec<Option<u16>> = CODIGO_DA_NATUREZA_BC.into_iter().map(Some).collect();

        println!("nats: {nats:?}");

        assert_eq!(nats[0], Some(1));
        assert_eq!(nats[17], Some(18));
    }

    #[test]
    fn map_to_option() {
        // cargo test -- --show-output map_to_option
        let nats = CODIGO_DA_NATUREZA_BC.map(Some);

        println!("nats: {nats:?}");

        assert_eq!(nats[0], Some(1));
        assert_eq!(nats[17], Some(18));
    }
}
