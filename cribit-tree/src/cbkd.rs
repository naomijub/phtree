/// `Key` dimensional `CriBit` tree Implementation over generic value of type `V`
pub trait CritBit<V> {
    type Key;

    /// Inserts `value: V` at `key: Key`
    /// # Params:
    /// - Tree key of type `Key` is user defined.
    /// - Tree value for `key`, of generic type V
    ///
    /// # Return
    /// - `Option<V>`.
    ///     * If return is `None`, `(Key, V)` were inserted without previous entries.
    ///     * If return is `Some(V)`, `(Key, V)` already had an entry, and previous `V` was returned.
    fn insert(&mut self, key: Self::Key, value: V) -> Option<V>;

    /// Checks if tree contains `key: Self::Key`
    /// # Params:
    /// - Tree key of type `Self::Key`, where `Key` is user defined.
    ///
    /// # Return
    /// - `bool`.
    ///     * If `true`, `key: Self::Key` was found in the tree
    ///     * If `false`, `key: Self::Key` was NOT found in the tree
    fn contains(&self, key: Self::Key) -> bool;

    /// Removes key-value pair at key. (`key: Self::Key`, `value: V`)
    /// # Params:
    /// - Tree key of type `Self::Key`, where `Key` is user defined.
    ///
    /// # Return
    /// - `Option<V>`.
    ///    * If return is `None`, `key: Self::Key` was not found in entries.
    ///    * If return is `Some(V)`, (`key: Self::Key`, `value: V`) was removed and `value: V` is returned.
    fn remove(&mut self, key: Self::Key) -> Option<V>;

    /// Removes key-value pair at key. (`key: Self::Key`, `value: V`)
    /// # Params:
    /// - Tree key of type `Self::Key`, where `Key` is user defined.
    ///
    /// # Return
    /// - `Option<V>`.
    ///    * If return is `None`, `key: Self::Key` was not found in entries.
    ///    * If return is `Some((Self::Key, V))`, (`key: Self::Key`, `value: V`) was removed and returned.
    fn remove_kv(&mut self, key: Self::Key) -> Option<(Self::Key, V)>;

    /// Returns the number of entries in the tree
    /// # Return
    /// - `usize`.
    fn len(&self) -> usize;

    /// Checks if the tree has no entries
    /// # Return
    /// - `bool`.
    ///    * true if it is empty.
    ///    * false if it is NOT empty. len > 0.
    fn is_empty(&self) -> bool;

    /// Gets reference of value at key. (`key: Self::Key`, `value: V`)
    /// # Params:
    /// - Tree key of type `Self::Key`, where `Key` is user defined.
    ///
    /// # Return
    /// - `Option<&V>`.
    ///    * If return is `None`, `key: Self::Key` was not found in entries.
    ///    * If return is `Some(&V)`, reference to `value: V` is returned.
    fn get(&self, key: Self::Key) -> Option<&V>;

    // /// Gets mutable reference of value at key. (`key: Self::Key`, `value: V`)
    // /// # Params:
    // /// - Tree key of type `Self::Key`, where `Key` is user defined.
    // ///
    // /// # Return
    // /// - `Option<&mut V>`.
    // ///     * If return is `None`, `key: Self::Key` was not found in entries.
    // ///     * If return is `Some(&mut V)`, mutable reference to `value: V` is returned.
    // fn get_mut(self, key: Self::Key) -> Option<&'static mut V>;
}

// /**
//  * @param min Lower left corner of the query window
//  * @param max Upper right corner of the query window
//  * @return Iterator over query result
//  *
//  * @see CritBit#query(long[], long[])
//  */
// QueryIterator<V> query(long[] min, long[] max);

// /**
//  * @return Iterator over all entries
//  *
//  * @see CritBit#iterator()
//  */
// FullIterator<V> iterator();
