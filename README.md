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

fn add_child(q: &Vec<i32>, i: i32, x: i32) {
    q[i] = x;
    let f = (i - 1) / 2;

    if q[f] < x {
       q[i] = q[f];
       q[f] = x;

       add_child(q, f, x);
    }
}

add_child(&q, q.lenght, 15);
```

```rust
// Pegar raiz
[21][16][9][7][9][4][8][6][1]

let q = vec![...];

fn get(q: &Vec<i32>) -> i32 {
    let x = q[0];

    q[0] = q[i];
    q[i] = 0;

    return x;
}

fn get_a(q: &Vec<i32>, i: i32) {

}
```

## Heapsort

```rust
fn heapsort(v: &[i32]) {

}
```

Heap: O(n)
Heapsort: O(n log n)
