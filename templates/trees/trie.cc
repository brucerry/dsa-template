/*
 * A trie stores words containing lowercase letters a through z.
 * Words with a common prefix share the same path from the root.
 */

#include <string_view>

struct Trie {
    Trie* children[26] = {};
    bool is_end = false;

    Trie() = default;

    void insert(std::string_view word) {
        Trie* cur = this;
        for (char c : word) {
            if (!cur->children[c - 'a'])
                cur->children[c - 'a'] = new Trie();
            cur = cur->children[c - 'a'];
        }
        cur->is_end = true;
    }

    Trie* search_prefix(std::string_view prefix) {
        Trie* cur = this;
        for (char c : prefix) {
            if (!cur->children[c - 'a'])
                return nullptr;
            cur = cur->children[c - 'a'];
        }
        return cur;
    }

    bool search(std::string_view word) {
        Trie* res = search_prefix(word);
        return res and res->is_end;
    }
};
