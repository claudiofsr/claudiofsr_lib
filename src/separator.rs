// ============================================================================
// thousands_separator
// ============================================================================

use rust_decimal::Decimal;

// 1. Criamos um Enum para definir os estilos disponíveis
pub enum FormatStyle {
    Euro, // 1.234,56 (Euro)
    PtBr, // 1.234,56 (Brasil)
    Us,   // 1,234.56 (EUA/Internacional)
}

// 2. Definimos uma Trait (interface) para unificar o comportamento
pub trait FormattableNumber {
    fn is_negative_num(&self) -> bool;
    fn format_abs(&self, decimals: usize) -> String;
}

// 3. Implementamos para f32
impl FormattableNumber for f32 {
    fn is_negative_num(&self) -> bool {
        self.is_sign_negative()
    }

    fn format_abs(&self, decimals: usize) -> String {
        format!("{:.1$}", self.abs(), decimals)
    }
}

// 4. Implementamos para f64
impl FormattableNumber for f64 {
    fn is_negative_num(&self) -> bool {
        self.is_sign_negative()
    }

    fn format_abs(&self, decimals: usize) -> String {
        format!("{:.1$}", self.abs(), decimals)
    }
}

// 5. Implementamos para Decimal
impl FormattableNumber for Decimal {
    fn is_negative_num(&self) -> bool {
        self.is_sign_negative()
    }

    fn format_abs(&self, decimals: usize) -> String {
        // Decimal já implementa Display respeitando a precisão
        format!("{:.1$}", self.abs(), decimals)
    }
}

// 6. A MÁGICA: Blanket Implementation para Referências
// Isso diz: "Implemente para &T, desde que T já saiba fazer"
impl<T: FormattableNumber> FormattableNumber for &T {
    fn is_negative_num(&self) -> bool {
        (*self).is_negative_num()
    }
    fn format_abs(&self, decimals: usize) -> String {
        (*self).format_abs(decimals)
    }
}

// 4. Função principal agora aceita T

/**
Formats a numeric value into a string with thousands separators and a specific number of decimal places.

This function supports various numeric types (`f32`, `f64`, `Decimal`) via the `FormattableNumber` trait
and applies formatting based on the selected `FormatStyle` (e.g., swapping dots and commas for Brazilian/European locales).

### Arguments

* `value`: The number to be formatted. It accepts values or references.
* `decimals`: The number of decimal places to include in the output.
* `style`: The `FormatStyle` enum determining the separators (e.g., `PtBr`, `Us`, `Euro`).

### Example

```rust
    use claudiofsr_lib::{thousands_separator, FormatStyle};

    let number = 1234567.8952;

    // Brazilian format: Dot for thousands, Comma for decimals
    let pt_br = thousands_separator(number, 2, FormatStyle::PtBr);
    assert_eq!(pt_br, "1.234.567,90"); // Note the rounding

    // US format: Comma for thousands, Dot for decimals
    let us = thousands_separator(number, 3, FormatStyle::Us);
    assert_eq!(us, "1,234,567.895");
```
*/
pub fn thousands_separator<T: FormattableNumber>(
    value: T,
    decimals: usize,
    style: FormatStyle,
) -> String {
    let round: String = value.format_abs(decimals);

    // Seleciona os separadores baseados no Enum
    let (thousands_sep, decimal_sep) = match style {
        FormatStyle::Euro => ('.', ","),
        FormatStyle::PtBr => ('.', ","),
        FormatStyle::Us => (',', "."),
    };

    // Lógica para separar inteiro de fração
    let (integer, fraction) = if decimals > 0 {
        // Encontra o ponto gerado pelo format! (sempre usa ponto internamente)
        if let Some(idx) = round.rfind('.') {
            (&round[..idx], Some(&round[idx + 1..]))
        } else {
            (round.as_str(), None)
        }
    } else {
        (round.as_str(), None)
    };

    let integer_splitted = split_and_insert(integer, thousands_sep);

    let result = if let Some(frac) = fraction {
        format!("{}{}{}", integer_splitted, decimal_sep, frac)
    } else {
        integer_splitted
    };

    if value.is_negative_num() {
        format!("-{}", result)
    } else {
        result
    }
}

/// Função auxiliar para inserir os pontos de milhar.
pub fn split_and_insert(integer: &str, separator: char) -> String {
    let len = integer.len();

    // Otimização de caso base: se a string for vazia, retorna imediatamente.
    if len == 0 {
        return String::new();
    }

    // --- 1. Estratégia de Pré-alocação (Heap Allocation) ---
    // Strings em Rust são vetores de bytes (Vec<u8>). Se não reservarmos espaço,
    // o vetor cresce dinamicamente, causando cópias de memória desnecessárias.

    // Garante o tamanho correto mesmo para caracteres Unicode multi-byte (ex: emojis, símbolos).
    let sep_len = separator.len_utf8();

    // Calcula quantos separadores serão inseridos.
    // Ex: "1000" (len 4) -> (3)/3 = 1 separador. "100" (len 3) -> (2)/3 = 0 separadores.
    let num_seps = (len - 1) / 3;

    // Calcula o tamanho final exato em bytes.
    let capacity = len + (num_seps * sep_len);

    // Cria a String com capacidade total reservada. Zero realocações durante o loop.
    let mut result = String::with_capacity(capacity);

    // --- 2. Iteração e Construção ---
    // Usamos chars() para iterar corretamente sobre caracteres Unicode.
    // enumerate() nos dá o índice atual (i).
    for (i, c) in integer.chars().enumerate() {
        // Lógica de inserção:
        // 1. i > 0: Nunca insere separador antes do primeiro dígito.
        // 2. (len - i): Calcula quantos dígitos faltam para acabar a string.
        // 3. is_multiple_of(3): Se o que falta é múltiplo de 3, é hora do separador.
        //    Nota: Assume-se que 'integer' contém apenas dígitos ASCII (0-9).
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(separator);
        }

        // Insere o dígito original
        result.push(c);
    }

    result
}

//----------------------------------------------------------------------------//
//                                   Tests                                    //
//----------------------------------------------------------------------------//
//
// cargo test -- --help
// cargo test -- --nocapture
// cargo test -- --show-output

#[cfg(test)]
mod separator_tests {
    use super::*;

    /// cargo test -- --show-output thousands_separator_test
    #[test]
    fn thousands_separator_test() {
        // Teste com referência de f32 (&f32)
        let val_f32: &f32 = &-5000.0;
        let result = thousands_separator(val_f32, 2, FormatStyle::PtBr);
        println!("f32: {val_f32}");
        println!("result: {result}\n");
        assert_eq!(result, "-5.000,00");

        // Teste com f64
        let val_f64: f64 = -1234567.8949;
        let result = thousands_separator(val_f64, 2, FormatStyle::PtBr);
        println!("f64: {val_f64}");
        println!("result: {result}\n");
        assert_eq!(result, "-1.234.567,89");

        // Teste com f64
        let val_f64: f64 = -1234567.8950;
        let result = thousands_separator(val_f64, 2, FormatStyle::PtBr);
        println!("f64: {val_f64}");
        println!("result: {result}\n");
        assert_eq!(result, "-1.234.567,90");

        // Teste com Decimal
        let val_decimal: Decimal = Decimal::new(12345678951, 4); // 1234567.8912
        let result = thousands_separator(val_decimal, 3, FormatStyle::PtBr);
        println!("decimal: {val_decimal}");
        println!("result: {result}\n");
        assert_eq!(result, "1.234.567,895");

        // Teste Estilo Americano
        let val_f64 = 1234567.8912;
        let result = thousands_separator(val_f64, 2, FormatStyle::Us);
        println!("f64: {val_f64}");
        println!("us result: {result}\n");
        assert_eq!(result, "1,234,567.89");
    }
}
