/// # 23.6 Exercise: Protobuf Parsing
///
/// In this exercise, you will build a parser for the **protobuf** binary
/// encoding. Don’t worry, it’s simpler than it seems! This illustrates a
/// common parsing pattern, passing slices of data. The underlying data
/// itself is never copied.
///
/// Fully parsing a **protobuf** message requires knowing the types of the
/// fields, indexed by their field numbers. That is typically provided in
/// a **proto** file. In this exercise, we’ll encode that information into
/// match statements in functions that get called for each field.
///
/// We’ll use the following proto:
///
/// ```
///
/// message PhoneNumber {
///     optional string number = 1;
///     optional string type = 2;
/// }
///  
/// message Person {
///     optional string name = 1;
///     optional int32 id = 2;
///     repeated PhoneNumber phones = 3;
/// }   
///
/// ```
///
/// A proto message is encoded as a series of fields, one after the next.
/// Each is implemented as a “tag” followed by the value. The tag contains
/// a field number (e.g., 2 for the id field of a Person message) and a
/// wire type defining how the payload should be determined from the byte
///  stream.
///
/// Integers, including the tag, are represented with a variable-length
/// encoding called `VARINT`. Luckily, parse_varint is defined for you
/// below. The given code also defines callbacks to handle `Person` and
/// `PhoneNumber` fields, and to parse a message into a series of calls to
/// those callbacks.
///
/// What remains for you is to implement the `parse_field` function.
///
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid varint")]
    InvalidVarint,
    #[error("Invalid wire-type")]
    InvalidWireType,
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    #[error("Invalid length")]
    InvalidSize(#[from] std::num::TryFromIntError),
    #[error("Unexpected wire-type)")]
    UnexpectedWireType,
    #[error("Invalid string (not UTF-8)")]
    InvalidString,
}

/// A wire type as seen on the wire.
enum WireType {
    /// The Varint WireType indicates the value is a single VARINT.
    Varint,
    //I64,  -- not needed for this exercise
    /// The Len WireType indicates that the value is a length represented as a VARINT
    /// followed by exactly that number of bytes.
    Len,
    /// The I32 WireType indicates that the value is precisely 4 bytes in little-endian order
    /// containing a 32-bit signed integer.
    I32,
}

#[derive(Debug)]
/// A field's value, typed based on the wire type.
enum FieldValue<'a> {
    Varint(u64),
    //I64(i64),  -- not needed for this exercise
    Len(&'a [u8]),
    I32(i32),
}

#[derive(Debug)]
/// A field, containing the field number and its value.
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

impl TryFrom<u64> for WireType {
    type Error = Error;

    fn try_from(value: u64) -> Result<WireType, Error> {
        Ok(match value {
            0 => WireType::Varint,
            //1 => WireType::I64,  -- not needed for this exercise
            2 => WireType::Len,
            5 => WireType::I32,
            _ => return Err(Error::InvalidWireType),
        })
    }
}

impl<'a> FieldValue<'a> {
    fn as_string(&self) -> Result<&'a str, Error> {
        let FieldValue::Len(data) = self else {
            return Err(Error::UnexpectedWireType);
        };
        std::str::from_utf8(data).map_err(|_| Error::InvalidString)
    }

    fn as_bytes(&self) -> Result<&'a [u8], Error> {
        let FieldValue::Len(data) = self else {
            return Err(Error::UnexpectedWireType);
        };
        Ok(data)
    }

    fn as_u64(&self) -> Result<u64, Error> {
        let FieldValue::Varint(value) = self else {
            return Err(Error::UnexpectedWireType);
        };
        Ok(*value)
    }
}

/// Parse a VARINT, returning the parsed value and the remaining bytes.
fn parse_varint(data: &[u8]) -> Result<(u64, &[u8]), Error> {
    for i in 0..7 {
        let Some(b) = data.get(i) else {
            return Err(Error::InvalidVarint);
        };
        if b & 0x80 == 0 {
            // This is the last byte of the VARINT, so convert it to
            // a u64 and return it.
            let mut value = 0u64;
            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return Ok((value, &data[i + 1..]));
        }
    }

    // More than 7 bytes is invalid.
    Err(Error::InvalidVarint)
}

/// Convert a tag into a field number and a WireType.
fn unpack_tag(tag: u64) -> Result<(u64, WireType), Error> {
    let field_num = tag >> 3;
    let wire_type = WireType::try_from(tag & 0x7)?;
    Ok((field_num, wire_type))
}

/// Parse a field, returning the remaining bytes
fn parse_field(data: &[u8]) -> Result<(Field, &[u8]), Error> {
    // 1. Read and unpack the tag.
    let (tag, remainder) = parse_varint(data)?;
    let (field_num, wire_type) = unpack_tag(tag)?;

    // 2. Based on the wire type, build a Field, consuming as many bytes as
    //    necessary.
    let (fieldvalue, remainder) = match wire_type {
        WireType::Varint => {
            let (value, remainder) = parse_varint(remainder)?;
            (FieldValue::Varint(value), remainder)
        }

        WireType::Len => {
            let (len, remainder) = parse_varint(remainder)?;
            let len: usize = len.try_into()?;
            if remainder.len() < len {
                return Err(Error::UnexpectedEOF);
            }
            let (value, remainder) = remainder.split_at(len);
            (FieldValue::Len(value), remainder)
        }

        WireType::I32 => {
            if remainder.len() < 4usize {
                return Err(Error::UnexpectedEOF);
            }
            let (value, remainder) = remainder.split_at(4);
            // Unwrap error because `value` is definitely 4 bytes long.
            let value = i32::from_le_bytes(value.try_into().unwrap());
            (FieldValue::I32(value), remainder)
        }
    };

    // 3. Return the field, and any un-consumed bytes.
    Ok((
        Field {
            field_num,
            value: fieldvalue,
        },
        remainder,
    ))

}

/// Parse a message in the given data, calling `field_callback` for each field in the message.
///
/// The entire input is consumed.
fn parse_message(
    mut data: &[u8],
    field_callback: impl Fn(Field) -> Result<(), Error>,
) -> Result<(), Error> {
    while !data.is_empty() {
        let parsed = parse_field(data)?;
        field_callback(parsed.0)?;
        data = parsed.1;
    }
    Ok(())
}

fn main() {
    /// Handle a field in a Person message.
    fn person_field(field: Field) -> Result<(), Error> {
        match field.field_num {
            1 => println!("name: {}", field.value.as_string()?),
            2 => println!("id: {}", field.value.as_u64()?),
            3 => {
                println!("phone:");
                parse_message(field.value.as_bytes()?, phone_number_field)?;
            }
            _ => {} // skip everything else
        }
        Ok(())
    }

    /// Handle a field in a PhoneNumber message.
    fn phone_number_field(field: Field) -> Result<(), Error> {
        match field.field_num {
            1 => println!("  number: {}", field.value.as_string()?),
            2 => println!("  type: {}", field.value.as_string()?),
            _ => {} // skip everything else
        }
        Ok(())
    }

    parse_message(
        &[
            0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a, 0x16, 0x0a,
            0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35, 0x2d, 0x31, 0x32, 0x31,
            0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a, 0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38,
            0x30, 0x30, 0x2d, 0x38, 0x36, 0x37, 0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d,
            0x6f, 0x62, 0x69, 0x6c, 0x65,
        ],
        person_field,
    )
    .unwrap()
}
