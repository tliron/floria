use {
    depiction::{markup::*, *},
    kutil::std::{immutable::*, iter::*},
    problemo::common::*,
    std::{convert::*, fmt, io, slice, str::*, vec},
};

/// Directory delimiter.
pub const DIRECTORY_DELIMITER: char = '/';

/// Directory delimiter.
pub const DIRECTORY_DELIMITER_STRING: &str = "/";

/// Invalid directory characters.
pub const INVALID_DIRECTORY_CHARACTERS: [char; 1] = [DIRECTORY_DELIMITER];

//
// Directory
//

/// Directory.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Directory(Vec<ByteString>);

// TODO: don't allow segments that contain "/" or ":"? escape them?

impl Directory {
    /// Constructor.
    pub fn new(segments: Vec<ByteString>) -> Result<Self, MalformedError> {
        for segment in &segments {
            for c in INVALID_DIRECTORY_CHARACTERS {
                if segment.contains(c) {
                    return Err(format!("directory segment contains invalid character: {}", c).into());
                }
            }
        }

        Ok(Self(segments))
    }

    /// Constructor.
    pub fn new_unchecked(segments: Vec<ByteString>) -> Self {
        Self(segments)
    }

    /// True if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add a segment to the start.
    pub fn add_first_segment(&mut self, segment: ByteString) {
        self.0.insert(0, segment);
    }

    /// Add a segment to the end.
    pub fn add_last_segment(&mut self, segment: ByteString) {
        self.0.push(segment);
    }
}

impl ToDepictionMarkup for Directory {
    fn to_depiction_markup(&self) -> String {
        let segments: Vec<String> =
            self.0.iter().map(|segment| format!("|meta|{}|", escape_depiction_markup(segment))).collect();
        segments.join(&format!("|delimiter|{}|", DIRECTORY_DELIMITER))
    }

    fn into_depiction_markup(self) -> String {
        self.to_depiction_markup()
    }
}

impl Depict for Directory {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        if !self.0.is_empty() {
            context.separate(writer)?;
            for (segment, last) in IterateWithLast::new(self) {
                context.theme.write_meta(writer, segment)?;
                if !last {
                    context.theme.write_delimiter(writer, DIRECTORY_DELIMITER)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0.join(DIRECTORY_DELIMITER_STRING))
    }
}

// Conversions

impl FromStr for Directory {
    type Err = MalformedError;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Self::new(representation.split(DIRECTORY_DELIMITER).map(|segment| segment.into()).collect())
    }
}

impl IntoIterator for Directory {
    type Item = ByteString;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'this> IntoIterator for &'this Directory {
    type Item = &'this ByteString;
    type IntoIter = slice::Iter<'this, ByteString>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
