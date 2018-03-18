use ast::ast::{
    Literal,
    Token
};

use span::Span;
use internal::ErrorKindExtension;

use nom::{
    InputIter,
    InputLength,
    Slice,
    IResult
};

/// Recognize a string.
pub fn string(span: Span) -> IResult<Span, Literal> {
    use nom::{
        Err,
        ErrorKind,
        Context,
        Needed
    };

    let input= span.as_slice();
    let mut iterator= input.char_indices();

    match iterator.next() {
        Some((index, item)) => {
            if item != '"' {
                return Err(Err::Error(Context::Code(span, ErrorKind::Custom(ErrorKindExtension::StringInvalidOpeningCharacter as u32))));
            }
        },
        None => return Err(Err::Incomplete(Needed::Size(2)))
    }

    let mut output: Option<String> = None;
    let mut offset = 1;
    let mut range;

    while let Some((index, item)) = iterator.next() {
        if item == '\\' {
            if let Some((next_index, next_item)) = iterator.next() {
                range = offset..index;

                if let None = output {
                    let mut data = (&input[range]).to_string();
                    data.push(next_item);
                    output = Some(data);
                } else if let Some(data) = output.as_mut() {
                    data.push_str(&input[range]);
                    data.push(next_item);
                }

                offset = next_index + 1;
            } else {
                return Err(Err::Incomplete(Needed::Size(1)));
            }
        } else if item == '"' {
            range = offset..index;

            if let None = output {
                output = Some((&input[range]).to_string());
            } else if let Some(data) = output.as_mut() {
                data.push_str(&input[range]);
            }

            return Ok((
                span.slice(index + 1..),
                Literal::String(Token::new(output.unwrap(), span.slice(..index + 1)))
            ));
        }
    }

    Err(Err::Incomplete(Needed::Size(1)))
}

#[cfg(test)]
mod tests {
    use ast::ast::{
        Literal,
        Token
    };

    use internal::{
        Context,
        Error,
        ErrorKind
    };

    use nom::{
        Needed,
        Err
    };

    use span::Span;
    use internal::ErrorKindExtension;

    use super::string;

    #[test]
    fn case_string() {
        let input  = Span::new("\"foobar\"tail");
        let output = Ok((
            Span::new_at("tail", 8, 1, 9),
            Literal::String(
                Token::new(
                    "foobar".to_string(),
                    Span::new("\"foobar\"")
                )
            )
        ));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_string_escaped_quote() {
        let input  = Span::new("\"foo\\\"bar\"tail");
        let output = Ok((
            Span::new_at("tail", 10, 1, 11),
            Literal::String(
                Token::new(
                    "foo\"bar".to_string(),
                    Span::new("\"foo\\\"bar\"")
                )
            )
        ));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_string_escaped_many() {
        let input  = Span::new("\"\\\"f\\oo\\\\bar\\\\\"tail");
        let output = Ok((
            Span::new_at("tail", 15, 1, 16),
            Literal::String(
                Token::new(
                    "\"foo\\bar\\".to_string(),
                    Span::new("\"\\\"f\\oo\\\\bar\\\\\"")
                )
            )
        ));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_string_empty() {
        let input  = Span::new("\"\"tail");
        let output = Ok((
            Span::new_at("tail", 2, 1, 3),
            Literal::String(
                Token::new(
                    "".to_string(),
                    Span::new("\"\"")
                )
            )
        ));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_invalid_string_empty() {
        let input  = Span::new("");
        let output = Err(Err::Incomplete(Needed::Size(2)));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_invalid_string_too_short() {
        let input  = Span::new("\"");
        let output = Err(Err::Incomplete(Needed::Size(1)));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_invalid_string_opening_character() {
        let input  = Span::new("foobar\"");
        let output = Err(Error::Error(Context::Code(input, ErrorKind::Custom(ErrorKindExtension::StringInvalidOpeningCharacter as u32))));

        assert_eq!(string(input), output);

    }

    #[test]
    fn case_invalid_string_closing_character() {
        let input  = Span::new("\"foobar");
        let output = Err(Err::Incomplete(Needed::Size(1)));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_string_utf8_japanese() {
        let input  = Span::new("\"出テル村七らぐし鏡始ヤユ権連ふこ応法マチスイ忽引コマヱシ際31意団ナ割平と上内すめリ気両え検抜爆じずラ。経イフシ兄町ユフヤ造審くー集57事め東福オセ会爆レコラハ健投ケ尾2連製テオロコ結踊ホヌラヘ購最や話催俊ち疑質メ都能坂居にぜ。南ば続壮ス韓再いき揃問業りぱ氷3連乗ぶあょ属保しぶ意者ル功並ょ末手第場ケサ教施セヤ式5幹回ッみぽ娘公賀待種男のせぽ\"");
        let output = Ok((
            Span::new_at("", 501, 1, 502),
            Literal::String(
                Token::new(
                    "出テル村七らぐし鏡始ヤユ権連ふこ応法マチスイ忽引コマヱシ際31意団ナ割平と上内すめリ気両え検抜爆じずラ。経イフシ兄町ユフヤ造審くー集57事め東福オセ会爆レコラハ健投ケ尾2連製テオロコ結踊ホヌラヘ購最や話催俊ち疑質メ都能坂居にぜ。南ば続壮ス韓再いき揃問業りぱ氷3連乗ぶあょ属保しぶ意者ル功並ょ末手第場ケサ教施セヤ式5幹回ッみぽ娘公賀待種男のせぽ".to_string(),
                    input
                )
            )
        ));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_string_utf8_russian() {
        let input  = Span::new("\"Лорем ипсум долор сит амет, еи вис хабео мутат, меа децоре десеруиссе ут, лорем интеллегат вим ан. Еа цивибус епицуреи атоморум вис, семпер ессент интегре иус ад. Еи вих регионе сцрипсерит. Но сеа ерат маиестатис, ерипуит детрахит ат нам, ад цонгуе волуптуа медиоцрем при\"");
        let output = Ok((
            Span::new_at("", 492, 1, 493),
            Literal::String(
                Token::new(
                    "Лорем ипсум долор сит амет, еи вис хабео мутат, меа децоре десеруиссе ут, лорем интеллегат вим ан. Еа цивибус епицуреи атоморум вис, семпер ессент интегре иус ад. Еи вих регионе сцрипсерит. Но сеа ерат маиестатис, ерипуит детрахит ат нам, ад цонгуе волуптуа медиоцрем при".to_string(),
                    input
                )
            )
        ));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_string_utf8_greek() {
        let input  = Span::new("\"Λορεμ ιπσθμ δολορ σιτ αμετ, ει σιτ ινvιδθντ εθριπιδισ ινcορρθπτε. Σεα αδ διcατ δολορ, εαμ cομμοδο νθσqθαμ σιγνιφερθμqθε εα. Μολλισ σινγθλισ ατ μει. Ερρορ δισσεντιθντ εξ μεα, θτ vισ ηινc vιδε, τε vελ πρινcιπεσ σcριπσεριτ. Αδ εαμ λθcιλιθσ παρτιενδο ιμπερδιετ, qθο πθτεντ τιμεαμ ετ\"");
        let output = Ok((
            Span::new_at("", 492, 1, 493),
            Literal::String(
                Token::new(
                    "Λορεμ ιπσθμ δολορ σιτ αμετ, ει σιτ ινvιδθντ εθριπιδισ ινcορρθπτε. Σεα αδ διcατ δολορ, εαμ cομμοδο νθσqθαμ σιγνιφερθμqθε εα. Μολλισ σινγθλισ ατ μει. Ερρορ δισσεντιθντ εξ μεα, θτ vισ ηινc vιδε, τε vελ πρινcιπεσ σcριπσεριτ. Αδ εαμ λθcιλιθσ παρτιενδο ιμπερδιετ, qθο πθτεντ τιμεαμ ετ".to_string(),
                    input
                )
            )
        ));

        assert_eq!(string(input), output);
    }

    #[test]
    fn case_string_long() {
        let lorem_ipsum = "Lorem ipsum dolor sit amet, nulla vel, vel quis pellentesque, ultricies taciti id sapien aliquam iaculis vitae, at luctus id, pellentesque elit magna imperdiet. Urna elit viverra tortor, vitae commodo in in ut venenatis, lectus justo culpa suscipit. Sem odio et donec penatibus pretium. Vel a auctor etiam libero, egestas lectus mauris donec, orci wisi nulla, potenti purus bibendum morbi id vitae quisque, dui pellentesque sed. Aliquam convallis est sagittis, dui leo amet ac diam cras, erat sed, ante wisi sodales turpis ipsum diamlorem, id nunc nulla. Occaecati eget placerat, vehicula esse integer orci arcu quam massa, ipsum libero donec sit tempus, nunc donec turpis morbi, nibh commodo nulla lacus venenatis. Ullamcorper fringilla eros est. Amet nulla in cras, elit praesent, id quisque justo tincidunt, fermentum quisque, rhoncus vehicula aenean sem ut. Sollicitudin sit sagittis eros mattis massa, at arcu, fringilla justo mi dolor dui justo, maecenas integer pellentesque orci vehicula. Nec commodo sit nulla tortor dolor sed. Volutpat ante proin feugiat et sit sed. Sociis quis quam justo, mollis donec morbi.\
            Pulvinar eros labore a integer, commodo pretium vel at lobortis tortor nostrum, amet lacinia aliquam. Tellus elit nascetur libero eu ut. Gravida justo sollicitudin vestibulum quam nunc, congue at aliquam nascetur erat, wisi nam pulvinar natoque elit egestas. Sodales massa, mus aliquip dolor id, varius nulla auctor potenti velit, odio et orci, et fusce pede aenean et aliquet vivamus. Nulla tempor dictum elit sed interdum posuere, vehicula rhoncus consequat iaculis, sit ornare lorem nam tristique leo nisl. Eget hymenaeos phasellus ac mi ut, et lorem etiam leo tellus mauris, porttitor tempor eget pulvinar. Nullam consequat adipiscing, nibh ac mi magna consequat, convallis justo viverra nec, mauris aliquam ultrices vulputate adipisci.\
            Amet sed, et diam etiam tempus rutrum, et nulla sodales et nam. Nam iaculis porta elit placerat, adipiscing tincidunt arcu tempus est cum. Ut id risus tincidunt, tincidunt donec pede hendrerit elit et rutrum. Lacus vulputate donec fringilla vel lacinia, aenean ad in vitae, luctus donec placerat quam laoreet sit mauris, mi ut in maecenas curabitur imperdiet. Eu rhoncus vivamus porttitor, aliquam euismod amet consectetur cum porta, placerat nec integer suspendisse, mauris imperdiet cras. Odio neque nunc conubia totam quis tempor, elementum magnis at. Vulputate non, orci consequat eu. Est rhoncus maecenas lacinia imperdiet.\
            Elementum ab neque, justo sit, pellentesque fusce nisl phasellus, asperiores lectus dolor lobortis litora elit. Ultricies rhoncus duis, donec sit lobortis eu. Consequuntur praesent. A curabitur tortor penatibus ornare libero magna, mi vehicula, a habitasse fusce sunt interdum vestibulum ante, libero sapien quisque tortor lectus id praesent. Sit laoreet sollicitudin sit vestibulum aenean, habitant sit lorem integer vivamus, sapien cras ad libero sapien lorem egestas, dolor maiores vitae metus sed arcu. Rhoncus consectetuer delectus viverra gravida ornare justo, ullamcorper egestas in in vulputate libero, egestas scelerisque pharetra velit nibh, a eu elit. Neque praesent, curabitur venenatis, mus ut leo enim. Eu rutrum pede viverra, et sem, lacus nullam non mi at varius sollicitudin, sem nec ornare nibh id justo ligula, amet orci etiam. Dolor magnis felis bibendum. Mollis laoreet augue, faucibus interdum lorem ultricies mattis per.";

        let mut input_data = "\"".to_string();
        input_data.push_str(lorem_ipsum);
        input_data.push_str("\"");

        let input  = Span::new(&input_data);

        let output = Ok((
            Span::new_at("", 3433, 1, 3434),
            Literal::String(
                Token::new(
                    lorem_ipsum.to_string(),
                    input
                )
            )
        ));

        assert_eq!(string(input), output);
    }



}