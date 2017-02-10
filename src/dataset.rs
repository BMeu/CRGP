//! Functions to determine which data set to use.

/// Possible data sets to run on.
pub enum DataSet {
    /// A small test set whose results can be checked manually.
    TestSet,

    /// A set consisting of 3500 retweets.
    RT3500Set,

    /// A set consisting of 7226 retweets.
    RT7226Set
}

impl DataSet {
    /// Determine from a given string ``arg`` which data set to run use.
    ///
    /// If ``arg`` is ``3500`` ``DataSet::RT3500Set`` will be used, if it is ``7226``
    /// ``DataSet::RT7226Set``, and in all other cases ``DataSet::TestSet``.
    pub fn from_string(arg: &str) -> DataSet {
        match arg {
            "3500" => DataSet::RT3500Set,
            "7226" => DataSet::RT7226Set,
            _ => DataSet::TestSet
        }
    }
}
