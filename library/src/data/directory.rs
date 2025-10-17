use {
    depiction::*,
    kutil::std::{immutable::*, iter::*},
    std::{convert::*, fmt, io, slice, str::*, vec},
};

//
// Directory
//

/// Directory.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Directory(pub Vec<ByteString>);

// TODO: don't allow segments that contain "/" or ":"? escape them?

impl Directory {
    /// Add a segment to the start.
    pub fn add_first_segment(&mut self, segment: ByteString) {
        self.0.insert(0, segment);
    }

    /// Add a segment to the end.
    pub fn add_last_segment(&mut self, segment: ByteString) {
        self.0.push(segment);
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
                write!(writer, "{}", context.theme.name_style.remove_all_effects().style(segment))?;
                if !last {
                    context.theme.write_delimiter(writer, '/')?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0.join("/"))
    }
}

// Conversions

impl FromStr for Directory {
    type Err = Infallible;

    fn from_str(representation: &str) -> Result<Self, Self::Err> {
        Ok(Self(representation.split('/').map(|segment| segment.into()).collect()))
    }
}

impl FromIterator<ByteString> for Directory {
    fn from_iter<IteratorT>(iterator: IteratorT) -> Self
    where
        IteratorT: IntoIterator<Item = ByteString>,
    {
        Self(iterator.into_iter().collect())
    }
}

impl IntoIterator for Directory {
    type Item = ByteString;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'own> IntoIterator for &'own Directory {
    type Item = &'own ByteString;
    type IntoIter = slice::Iter<'own, ByteString>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
