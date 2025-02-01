use regex::Regex;

pub(crate) fn replace_script_tags(input: &str) -> String {
    let sup_regex = Regex::new(r"<sup>([^<]+)</sup>").unwrap();
    let sub_regex = Regex::new(r"<sub>([^<]+)</sub>").unwrap();

    let sup_replaced = sup_regex.replace_all(input, |caps: &regex::Captures| {
        convert_to_script(&caps[1], true)
    });

    sub_regex
        .replace_all(&sup_replaced, |caps: &regex::Captures| {
            convert_to_script(&caps[1], false)
        })
        .to_string()
}

fn convert_to_script(text: &str, is_superscript: bool) -> String {
    text.chars()
        .map(|c| match (c, is_superscript) {
            ('0', true) => '⁰',
            ('0', false) => '₀',
            ('1', true) => '¹',
            ('1', false) => '₁',
            ('2', true) => '²',
            ('2', false) => '₂',
            ('3', true) => '³',
            ('3', false) => '₃',
            ('4', true) => '⁴',
            ('4', false) => '₄',
            ('5', true) => '⁵',
            ('5', false) => '₅',
            ('6', true) => '⁶',
            ('6', false) => '₆',
            ('7', true) => '⁷',
            ('7', false) => '₇',
            ('8', true) => '⁸',
            ('8', false) => '₈',
            ('9', true) => '⁹',
            ('9', false) => '₉',
            ('+', true) => '⁺',
            ('+', false) => '₊',
            ('-', true) => '⁻',
            ('-', false) => '₋',
            ('=', true) => '⁼',
            ('=', false) => '₌',
            ('(', true) => '⁽',
            ('(', false) => '₍',
            (')', true) => '⁾',
            (')', false) => '₎',
            ('n', true) => 'ⁿ',
            ('a', false) => 'ₐ',
            ('i', true) => 'ⁱ',
            ('e', false) => 'ₑ',
            _ => c,
        })
        .collect()
}
