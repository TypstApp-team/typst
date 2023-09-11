use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};

use ecow::eco_format;
use once_cell::sync::Lazy;

use super::{cast, func, Func, NativeFuncData, Scope, Value};
use crate::diag::StrResult;
use crate::util::Static;

#[doc(inline)]
pub use typst_macros::{scope, ty};

/// Describes a kind of value.
///
/// To style your document, you need to work with values of different kinds:
/// Lengths specifying the size of your elements, colors for your text and
/// shapes, and more. Typst categorizes these into clearly defined _types_ and
/// tells you where it expects which type of value.
///
/// Apart from very basic types for numeric values and [typical]($int)
/// [types]($float) [known]($str) [from]($array) [programming]($dictionary)
/// languages, Typst provides a special type for [_content._]($content) A value
/// of this type can hold anything that you can enter into your document: Text,
/// elements like headings and shapes, and style information.
#[ty(scope)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type(Static<NativeTypeData>);

impl Type {
    /// Get the type for `T`.
    pub fn of<T: NativeType>() -> Self {
        T::ty()
    }

    /// The type's short name, how it is used in code (e.g. `str`).
    pub fn short_name(&self) -> &'static str {
        self.0.name
    }

    /// The type's long name, for use in diagnostics (e.g. `string`).
    pub fn long_name(&self) -> &'static str {
        self.0.long_name
    }

    /// The type's title case name, for use in documentation (e.g. `String`).
    pub fn title(&self) -> &'static str {
        self.0.title
    }

    /// Documentation for the type (as Markdown).
    pub fn docs(&self) -> &'static str {
        self.0.docs
    }

    /// Search keywords for the type.
    pub fn keywords(&self) -> &'static [&'static str] {
        self.0.keywords
    }

    /// This type's constructor function.
    pub fn constructor(&self) -> StrResult<Func> {
        self.0
            .constructor
            .as_ref()
            .map(|lazy| Func::from(*lazy))
            .ok_or_else(|| eco_format!("type {self} does not have a constructor"))
    }

    /// The type's associated scope of sub-definition.
    pub fn scope(&self) -> &'static Scope {
        &(self.0).0.scope
    }

    /// Get a field from this type's scope, if possible.
    pub fn field(&self, field: &str) -> StrResult<&'static Value> {
        self.scope()
            .get(field)
            .ok_or_else(|| eco_format!("type {self} does not contain field `{}`", field))
    }
}

#[scope]
impl Type {
    /// Determines a value's type.
    ///
    /// ```example
    /// #type(12) \
    /// #type(14.7) \
    /// #type("hello") \
    /// #type(<glacier>) \
    /// #type([Hi]) \
    /// #type(x => x + 1) \
    /// #type(type)
    /// ```
    #[func(constructor)]
    pub fn construct(
        /// The value whose type's to determine.
        value: Value,
    ) -> Type {
        value.ty()
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(self.long_name())
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(self.long_name())
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.long_name().cmp(other.long_name())
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A Typst type that is defined by a native Rust type.
pub trait NativeType {
    /// The type's name.
    ///
    /// In contrast to `data()`, this is usable in const contexts.
    const NAME: &'static str;

    /// Get the type for the native Rust type.
    fn ty() -> Type {
        Type::from(Self::data())
    }

    // Get the type data for the native Rust type.
    fn data() -> &'static NativeTypeData;
}

/// Defines a native type.
pub struct NativeTypeData {
    pub name: &'static str,
    pub long_name: &'static str,
    pub title: &'static str,
    pub docs: &'static str,
    pub keywords: &'static [&'static str],
    pub constructor: Lazy<Option<&'static NativeFuncData>>,
    pub scope: Lazy<Scope>,
}

impl From<&'static NativeTypeData> for Type {
    fn from(data: &'static NativeTypeData) -> Self {
        Self(Static(data))
    }
}

cast! {
    &'static NativeTypeData,
    self => Type::from(self).into_value(),
}
