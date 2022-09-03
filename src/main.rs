use rastd::trie::HashTrie;

fn main() {
    let mut trie = HashTrie::new();
    trie.insert("foobar", 3);
    trie.insert("foobar", 5);
    dbg!(trie);
}
