use std::cmp::Ordering;
use std::fmt::Debug;

/// Узел AVL-дерева.
#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    height: usize,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K: Ord + Debug, V: Debug> Node<K, V> {
    /// Создает новый узел.
    fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    /// Получает высоту узла.
    fn height(node: &Option<Box<Node<K, V>>>) -> usize {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    /// Обновляет высоту узла.
    fn update_height(&mut self) {
        let left_height = Self::height(&self.left);
        let right_height = Self::height(&self.right);
        self.height = 1 + std::cmp::max(left_height, right_height);
    }

    /// Баланс-фактор узла.
    fn balance_factor(&self) -> isize {
        let left_height = Self::height(&self.left) as isize;
        let right_height = Self::height(&self.right) as isize;
        left_height - right_height
    }

    /// Выполняет правый поворот.
    fn rotate_right(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self
            .left
            .take()
            .expect("Левый дочерний узел должен существовать");
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(self);
        new_root.update_height();
        new_root
    }

    /// Выполняет левый поворот.
    fn rotate_left(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self
            .right
            .take()
            .expect("Правый дочерний узел должен существовать");
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(self);
        new_root.update_height();
        new_root
    }

    /// Балансирует узел.
    fn balance(mut self: Box<Self>) -> Box<Self> {
        self.update_height();
        let balance = self.balance_factor();

        // Левая тяжесть
        if balance > 1 {
            if self.left.as_ref().unwrap().balance_factor() < 0 {
                self.left = self.left.take().map(|left| left.rotate_left());
            }
            return self.rotate_right();
        }

        // Правая тяжесть
        if balance < -1 {
            if self.right.as_ref().unwrap().balance_factor() > 0 {
                self.right = self.right.take().map(|right| right.rotate_right());
            }
            return self.rotate_left();
        }

        self
    }

    /// Вставляет новый узел в поддерево.
    fn insert(mut self: Box<Self>, key: K, value: V) -> Box<Self> {
        match key.cmp(&self.key) {
            Ordering::Less => {
                if let Some(left) = self.left.take() {
                    self.left = Some(left.insert(key, value));
                } else {
                    self.left = Some(Box::new(Node::new(key, value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = self.right.take() {
                    self.right = Some(right.insert(key, value));
                } else {
                    self.right = Some(Box::new(Node::new(key, value)));
                }
            }
            Ordering::Equal => {
                self.value = value;
            }
        }
        self.balance()
    }

    /// Находит минимальный узел в поддереве.
    fn find_min(node: Box<Self>) -> Box<Self> {
        match node.left {
            Some(left) => Node::find_min(left),
            None => node,
        }
    }

    /// Удаляет узел с заданным ключом из поддерева.
    fn remove(mut self: Box<Self>, key: &K) -> Option<Box<Self>> {
        match key.cmp(&self.key) {
            Ordering::Less => {
                if let Some(left) = self.left.take() {
                    self.left = left.remove(key);
                }
            }
            Ordering::Greater => {
                if let Some(right) = self.right.take() {
                    self.right = right.remove(key);
                }
            }
            Ordering::Equal => {
                if self.left.is_none() {
                    return self.right;
                }
                if self.right.is_none() {
                    return self.left;
                }
                let min = Node::find_min(self.right.take().unwrap());
                self.key = min.key;
                self.value = min.value;
                self.right = min.right;
                self.left = min.left;
            }
        }
        Some(self.balance())
    }

    /// Ищет узел с заданным ключом.
    fn find(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Less => self.left.as_ref().and_then(|left| left.find(key)),
            Ordering::Greater => self.right.as_ref().and_then(|right| right.find(key)),
            Ordering::Equal => Some(&self.value),
        }
    }

    /// Итерация по узлам поддерева.
    fn inorder_traversal(&self, visit: &mut dyn FnMut(&K, &V)) {
        if let Some(ref left) = self.left {
            left.inorder_traversal(visit);
        }
        visit(&self.key, &self.value);
        if let Some(ref right) = self.right {
            right.inorder_traversal(visit);
        }
    }
}

/// Структура AVL-дерева.
pub struct AVLTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord + Debug, V: Debug> AVLTree<K, V> {
    /// Создает новое пустое AVL-дерево.
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    /// Вставляет узел с заданным ключом и значением.
    ///
    /// Если узел с таким ключом уже существует, его значение обновляется.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(root) = self.root.take() {
            self.root = Some(root.insert(key, value));
        } else {
            self.root = Some(Box::new(Node::new(key, value)));
        }
    }

    /// Удаляет узел с заданным ключом.
    ///
    /// Возвращает `true`, если узел был найден и удален, иначе `false`.
    pub fn remove(&mut self, key: &K) -> bool {
        if let Some(root) = self.root.take() {
            let new_root = root.remove(key);
            self.root = new_root;
            true
        } else {
            false
        }
    }

    /// Ищет значение по заданному ключу.
    ///
    /// Возвращает ссылку на значение, если ключ найден, иначе `None`.
    pub fn find(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|root| root.find(key))
    }

    /// Выполняет итерацию по всем узлам дерева в порядке возрастания ключей.
    ///
    /// Функция `visit` вызывается для каждого узла с ссылками на его ключ и значение.
    pub fn inorder_traversal<F>(&self, mut visit: F)
    where
        F: FnMut(&K, &V),
    {
        if let Some(ref root) = self.root {
            root.inorder_traversal(&mut visit);
        }
    }
}

impl<K: Ord + Debug, V: Debug> Default for AVLTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_find() {
        let mut tree = AVLTree::new();
        tree.insert(10, "a");
        tree.insert(20, "b");
        tree.insert(5, "c");

        assert_eq!(tree.find(&10), Some(&"a"));
        assert_eq!(tree.find(&20), Some(&"b"));
        assert_eq!(tree.find(&5), Some(&"c"));
        assert_eq!(tree.find(&15), None);
    }

    #[test]
    fn test_remove() {
        let mut tree = AVLTree::new();
        tree.insert(10, "a");
        tree.insert(20, "b");
        tree.insert(5, "c");
        tree.insert(15, "d");

        assert!(tree.remove(&10));
        assert!(tree.remove(&100));
    }
    #[test]
    fn test_inorder_traversal() {
        let mut tree = AVLTree::new();
        let elements = vec![(10, "a"), (20, "b"), (5, "c"), (15, "d")];
        for &(k, v) in &elements {
            tree.insert(k, v);
        }

        let mut result = Vec::new();
        tree.inorder_traversal(|k, v| result.push((*k, *v)));
        let expected = vec![(5, "c"), (10, "a"), (15, "d"), (20, "b")];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_value() {
        let mut tree = AVLTree::new();
        tree.insert(10, "a");
        assert_eq!(tree.find(&10), Some(&"a"));
        tree.insert(10, "updated");
        assert_eq!(tree.find(&10), Some(&"updated"));
    }
}
