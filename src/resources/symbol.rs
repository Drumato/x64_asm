use crate::resources::Group;
use elf_utilities::symbol;

#[derive(Eq, Ord, PartialOrd, PartialEq, Debug, Clone)]
pub struct Symbol {
    pub labels: Vec<Group>,
    /// Symbol Visibility(GLOBAL/LOCAL/etc.)
    pub visibility: u8,
    /// Symbol Type(NOTYPE/FUNCTION/etc.)
    pub ty: u8,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            labels: Vec::new(),
            ty: 0,
            visibility: symbol::STB_LOCAL,
        }
    }
}

impl Symbol {
    /// ```
    /// use x64_asm::Symbol;
    ///
    /// let mut s : Symbol = Default::default();
    /// assert!(!s.is_function());
    ///
    /// s.as_function();
    /// assert!(s.is_function());
    /// ```
    pub fn as_function(&mut self) {
        self.ty = symbol::STT_FUNC;
    }


    /// ```
    /// use x64_asm::Symbol;
    ///
    /// let mut s : Symbol = Default::default();
    /// assert!(!s.is_global());
    ///
    /// s.as_global();
    /// assert!(s.is_global());
    /// ```
    pub fn as_global(&mut self) {
        self.visibility = symbol::STB_GLOBAL;
    }

    pub fn is_function(&self) -> bool {
        (self.ty & symbol::STT_FUNC) != 0
    }

    pub fn is_global(&self) -> bool {
        (self.visibility & symbol::STB_GLOBAL) != 0
    }
}