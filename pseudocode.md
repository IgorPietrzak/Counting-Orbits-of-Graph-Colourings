# McKay Refinement Algorithm.

```rust
// Algorithm implies that pi and a are the same data structure!
// Also theorem 2.6 implies we can put pi as the pi and a parameter,
// making R(G,pi,pi) a valid object.

fn refine(G,pi,a) {

// STEP 1:
// Initialise variables
  let zeta = pi; // the eventual output
  let m = 1; // used to iterate over a.
  let M = a.len(); 

  /*
  If zeta is discrete we refined it to the max and cannot get
  any finer - it is trivially equitable.

  Other reason to stop is if we have gone through all the cells in a.

  */ 

 // STEP 2:

  if zeta.is_discrete() || m > M {
    return zeta;
  }

  let W = a[m]; // Take mth cell.
  
  m = m + 1; // Increase cell iterator
  
  let k = 1; // Iterator for cells of pi

  // STEP 3:
  // STEP 3 is a for loop over elements of zeta
  // zeta = (V1, ..., Vr); - look at step 4 (k <= r)

  /*
    Partition of Vk such that

     * X1 has vertices in Vk of lowest degree
     * Xs has vertices in Vk of highest degree
     * Degrees of vertices in Xi dont necesserily need
       to be the same.
  */

  let p_of_Vk = (X1, ..., Xs); 
  // defining cells for vertices of each degree w.r.t W will work here.

  
  // Only true when p_of_Vk = (Vk), also they have the same degree?
  if s == 1 {
    go to step 4
  }

  // pick first largest set in p_of_Vk
  
  let Xt = max(p_of_Vk);

  if a[j] == zeta[k] for some m <= j <= M {
    a[j] = Xt;
  }

  for i in 1 <= i < t {
    a[M + i] = p_of_Vk[i];
  }

  for i in t < i <= s {
    a[M + i - 1] = p_of_Vk[i];
  }

  M = M + s - 1;

  // Update zeta


  // STEP4:

  k = k + 1;

  if k <= r {
    go to 3;
  }

  go to 2;
  

  

  
}
  
```
