use serde::{Deserialize, Serialize};

/// Represents a row.  This has a length of some sort (optional) and a vector
/// of children.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(rename = "row")]
pub struct LayoutRow {
    pub child: Option<Vec<LayoutRowChild>>,
    pub ratio: Option<u32>,
}

/// Represents a child of a Row - either a Col (column) or a FinalWidget.
///
/// A Col can also have an optional length and children.  We only allow columns
/// to have FinalWidgets as children, lest we get some amount of mutual
/// recursion between Row and Col.
#[derive(Clone, Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum LayoutRowChild {
    Widget(FinalWidget),
    /// The first one in the list is the "default" selected widget.
    Carousel {
        carousel_children: Vec<String>,
        default: Option<bool>,
    },
    LayoutCol {
        ratio: Option<u32>,
        child: Vec<FinalWidget>,
    },
}

/// Represents a widget.
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct FinalWidget {
    #[serde(flatten)]
    pub rule: Option<LayoutRule>,
    #[serde(rename = "type")]
    pub widget_type: String,
    pub default: Option<bool>,
}

/// A "rule" denoting how this component is to be laid out.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(untagged)]
pub enum LayoutRule {
    /// Let the child decide how big to make the current node.
    Child,

    /// Expand to whatever space is left; the `ratio` determines how
    /// much space to take if there are more than one
    /// [`LayoutRule::Expand`] component.
    Expand { ratio: u32 },

    /// Take up exactly `length` space if possible.
    Length { length: u16 },
}

impl Default for LayoutRule {
    fn default() -> Self {
        LayoutRule::Expand { ratio: 1 }
    }
}
