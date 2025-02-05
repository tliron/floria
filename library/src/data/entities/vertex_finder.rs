use super::super::{directory::*, expression::*};

use {
    kutil::cli::depict::*,
    std::{collections::*, io},
};

//
// VertexFinder
//

/// Vertex finder.
#[derive(Clone, Debug)]
pub struct VertexFinder {
    /// Optional directories.
    pub directories: Option<Vec<Directory>>,

    /// Finder.
    pub finder: Call,
}

impl VertexFinder {
    /// Constructor.
    pub fn new(filter: Call) -> Self {
        Self { directories: None, finder: filter }
    }
}

impl Depict for VertexFinder {
    fn depict<WriteT>(&self, writer: &mut WriteT, context: &DepictionContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        match &self.directories {
            Some(_directories) => todo!(),
            None => self.finder.depict(writer, context),
        }
    }
}

impl Into<Expression> for VertexFinder {
    fn into(self) -> Expression {
        let mut map = BTreeMap::default();

        if let Some(directories) = self.directories {
            let mut directories_list = Vec::with_capacity(directories.len());
            for directory in directories {
                let directory: Vec<_> = directory.into_iter().map(|segment| segment.into()).collect();
                directories_list.push(directory.into());
            }
            map.insert("directories".into(), directories_list.into());
        }

        map.insert("finder".into(), self.finder.into());

        Expression::Map(map)
    }
}
