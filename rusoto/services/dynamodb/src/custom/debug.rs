use crate::generated::AttributeValue;
use std::fmt;

impl fmt::Debug for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! for_each_ident {
            ($($ident:ident),+; $($callback:tt)*) => {
                macro_rules! call {
                    $($callback)*
                }
                $(call!($ident);)*
            }
        }

        let AttributeValue {
            b,
            bool,
            bs,
            l,
            m,
            n,
            ns,
            null,
            s,
            ss,
        } = self;

        for_each_ident!(
            b,
            bool,
            bs,
            n,
            ns,
            null,
            s,
            ss;
            ($ident:ident) => {
                if let Some(val) = $ident {
                    return f.debug_tuple(stringify!($ident))
                        .field(&val)
                        .finish();
                }
            }
        );

        // Map and list are special, because they contain nested AttributeValue
        // that need to be wrapped too

        if let Some(val) = m {
            return f.debug_map().entries(val).finish();
        }

        if let Some(val) = l {
            return f.debug_list().entries(val).finish();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn showcase() {
        let value = AttributeValue {
            s: Some("blackjack".to_owned()),
            ..Default::default()
        };

        let expected_custom = expect![[r#"
            s(
                "blackjack",
            )
        "#]];

        expected_custom.assert_debug_eq(&value);
    }
}
