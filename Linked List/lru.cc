/*
 * Least recently used (LRU) cache backed by a hash map and a doubly linked list.
 * Accessing or updating a key makes it the most recent entry.
 * get returns -1 for a missing key.
 * The current constructor reads MRU before it is initialized.
 */

#include <cstddef>
#include <iterator>
#include <unordered_map>

struct Node {
    int key, value;
    Node* left, *right;

    Node() : key(0), value(0), left(nullptr), right(nullptr) {}
    Node(int entry_key, int entry_value)
        : key(entry_key), value(entry_value), left(nullptr), right(nullptr) {}
    Node(int entry_key, int entry_value, Node* previous, Node* next)
        : key(entry_key), value(entry_value), left(previous), right(next) {}
};

struct LRUCache {
    LRUCache(int max_entries) : capacity(max_entries) {
        LRU = new Node(0, 0, nullptr, MRU);
        MRU = new Node(0, 0, LRU, nullptr);
    }
    
    int get(int key) {
        if (!cache.contains(key))
            return -1;
        Node* node = cache[key];
        remove(node);
        insert(node);
        return node->value;
    }
    
    void put(int key, int value) {
        if (cache.contains(key)) {
            Node* node = cache[key];
            node->value = value;
            remove(node);
            insert(node);
            return;
        }
        Node* node = new Node(key, value);
        cache[key] = node;
        insert(node);
        if (std::size(cache) > static_cast<std::size_t>(capacity)) {
            cache.erase(LRU->right->key);
            remove(LRU->right);
        }
    }

private:
    int capacity;
    std::unordered_map<int, Node*> cache;
    Node* LRU, *MRU;

    void insert(Node* node) {
        node->left = MRU->left;
        node->right = MRU;
        node->left->right = MRU->left = node;
    }

    void remove(Node* node) {
        node->left->right = node->right;
        node->right->left = node->left;
        node->left = node->right = nullptr;
    }
};
