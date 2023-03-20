# Filas de Prioridade

Inserir, remover o elemento de maior prioridade.

-   [Exemplo](https://github.com/kayky-cas/heap)

## Heaps

Árvore

### Max-Heap

-   pai >= filho

### Min-Heap

-   pai <= filho

### Heap sem árvore

#### Regras

1. A raíz é o elemento na posição 0.
2. O filho esquerdo de um elemento na posição **i** está na posição **2i + 1**.
3. O filho direito de um elemento na posição **i** está na posição **2i + 2**.
4. O pai de quem está na posição **i**, está na posição **|(i - 1) / 2|**

##### Exemplo

```c
[3][1][4][5][9][2][6][5] // não é um max-heap
```

```rust
// Completo:
[21][16][9][7][9][4][8][6][1]
```

```rust
// Adicionar:
[15] -> [21][16][9][7][9][4][8][6][1]

let q = vec![...];

fn insert(queue: Vec<i32>, value: i32) {
    queue.push(value);

    if queue.len() == 1 {
        return;
    }

    sift_up(queue.len() - 1, value);
}

fn sift_up(queue: [i32], pos: usize, value: i32) {
    let father_pos = parent!(pos);

    if queue[father_pos] < value {
       queue[pos] = queue[father_pos];
       queue[father_pos] = value;

        if father_pos > 0 {
            sift_up(queue, father_pos, value);
        }
    }
}

sift_up(q.len() - 1, value);
```

```rust
// Pegar raiz
[21][16][9][7][9][4][8][6][1]

let q = vec![...];

fn get(queue: Vec<i32>) -> Option<i32> {
    if queue.is_empty() {
        return None;
    }

    let val = queue[0];

    sift_down(0);
    queue.pop();

    return Some(val);
}

fn sift_down(queue: Vec<i32>, pos: usize) {
    let mut pos_child = 2 * pos + 1;

    if pos_child > queue.len() - 1 {
        return;
    }

    let pos_right = pos_child + 1;

    if pos_right < queue.len() && queue[pos_right] > self.queue[pos_child] {
        pos_child = pos_right;
    }

    queue[pos] = queue[pos_child];
    sift_down(pos_child);
}
```

## Heapsort

```rust
fn heapsort(v: &[i32]) {
    for i in (0..=parent!(v.len() - 1)).rev() {
        sift_down(v, i);
    }
}
```

-   Heap: O(n)
-   Heapsort: O(n log n)
