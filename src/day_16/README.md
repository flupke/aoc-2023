# Optimization

## First attempt

The first implementation is a simple semi-recursive ray-marching algorithm.
It avoids infinite loops with a memo of seen rays:

```rust
fn do_trace(&self, ray: Ray, photon_map: &mut PhotonMap, seen: &mut HashSet<Ray>) {
    let mut ray = ray;
    loop {
        if ray.origin.x < 0
            || ray.origin.x as usize >= self.tiles.width
            || ray.origin.y < 0
            || ray.origin.y as usize >= self.tiles.height
            || seen.contains(&ray)
        {
            break;
        }
        let tile = *self.tiles.get(ray.origin);
        photon_map.add_photon(ray.origin);
        seen.insert(ray.clone());

        if tile == '.' {
            ray = ray.walk();
        } else if tile == '/' || tile == '\\' {
            ray = reflect(ray, tile)
        } else if tile == '|' || tile == '-' {
            let refracted = refract(&ray, tile);
            ray = refracted[0].clone();
            if refracted.len() == 2 {
                self.do_trace(refracted[1].clone(), photon_map, seen);
            }
        }
    }
}
```

It runs in ~237ms on a Ryzen 5950x:

```text
Benchmark 1: cargo run -r solve 16
  Time (mean ± σ):     237.3 ms ±   2.9 ms    [User: 212.2 ms, System: 24.0 ms]
  Range (min … max):   232.2 ms … 243.9 ms    12 runs
```

## Profiling

Most of the time seems to be spent in `HashMap`:

```text
Samples: 16K of event 'cycles:Pu', Event count (approx.): 853799311
Children Self Command Shared Object Symbol

- 31,97% 31,95% aoc-2023-rust-f aoc-2023-rust-flupke [.] core::hash::BuildHasher::hash_one ▒
- 29,78% 29,76% aoc-2023-rust-f aoc-2023-rust-flupke [.] _ZN71_$LT$core..hash..sip..Hasher$LT$S$GT$$u20$as$u20$core..hash..Hasher$GT$5write17h22efaee90◆
- 15,47% 10,96% aoc-2023-rust-f aoc-2023-rust-flupke [.] \_ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h5d256ff5f6b3aecbE.llvm.155599435039▒
- 10,70% 10,69% aoc-2023-rust-f aoc-2023-rust-flupke [.] aoc_2023_rust_flupke::day_16::MirrorMap::do_trace ▒
- 10,04% 10,03% aoc-2023-rust-f aoc-2023-rust-flupke [.] hashbrown::map::HashMap<K,V,S,A>::insert
```

## Removing recursion

Removing recursion gives almost no benefit, although the code seems easier to
follow and avoids the recursion function:

```rust
fn trace(&self, ray: Ray) -> PhotonMap {
    let mut ray = ray;
    let mut photon_map = PhotonMap::new(self.tiles.width, self.tiles.height);
    let mut ray_stack = Vec::new();
    let mut seen = HashSet::new();
    loop {
        if ray.origin.x < 0
            || ray.origin.x as usize >= self.tiles.width
            || ray.origin.y < 0
            || ray.origin.y as usize >= self.tiles.height
            || seen.contains(&ray)
        {
            if ray_stack.is_empty() {
                break;
            }
            ray = ray_stack.pop().unwrap();
        } else {
            let tile = *self.tiles.get(ray.origin);
            photon_map.add_photon(ray.origin);
            seen.insert(ray.clone());

            if tile == '.' {
                ray = ray.walk();
            } else if tile == '/' || tile == '\\' {
                ray = reflect(ray, tile)
            } else if tile == '|' || tile == '-' {
                let refracted = refract(&ray, tile);
                ray = refracted[0].clone();
                if refracted.len() == 2 {
                    ray_stack.push(refracted[1].clone());
                }
            }
        }
    }
    photon_map
}
```

```text
Benchmark 1: cargo run -r solve 16
  Time (mean ± σ):     229.9 ms ±   5.0 ms    [User: 204.7 ms, System: 23.8 ms]
  Range (min … max):   224.2 ms … 240.8 ms    13 runs
```

## Reducing vectors size

Changing all vector types from `Vector<i32>` to `Vector<i16>` gives a little 9%
boost, probably because of faster hashing (which amounts to 30% of runtime).

```text
Benchmark 1: cargo run -r solve 16
  Time (mean ± σ):     211.7 ms ±   5.5 ms    [User: 191.5 ms, System: 18.2 ms]
  Range (min … max):   206.4 ms … 223.6 ms    13 runs
```

## Reducing map size

Changing the mirrors map from an `Array<char>` to an `Array<u8>` actually gives 3%
less performance:

```text
Benchmark 1: cargo run -r solve 16
  Time (mean ± σ):     217.7 ms ±   1.5 ms    [User: 199.4 ms, System: 17.5 ms]
  Range (min … max):   215.7 ms … 220.7 ms    13 runs
```

This is probably due to memory alignment issues. I don't want to over-complicate `Array` so I just reverted this change.

## Conclusion

The optimized version now runs 11% faster than the original, and the `HashMap`
now takes even more time:

```text
Samples: 15K of event 'cycles:Pu', Event count (approx.): 799423175
  Children      Self  Command          Shared Object         Symbol
+   33,94%    33,92%  aoc-2023-rust-f  aoc-2023-rust-flupke  [.] core::hash::BuildHasher::hash_one
+   27,70%    27,68%  aoc-2023-rust-f  aoc-2023-rust-flupke  [.] _ZN71_$LT$core..hash..sip..Hasher$LT$S$GT$$u20$as$u20$core..hash..Hasher$GT$5write17h4bb7b7de6f
+   14,08%    12,52%  aoc-2023-rust-f  aoc-2023-rust-flupke  [.] _ZN9hashbrown3raw21RawTable$LT$T$C$A$GT$14reserve_rehash17h0cb3fd1442fef126E.llvm.1366797186149
+   11,73%    11,73%  aoc-2023-rust-f  aoc-2023-rust-flupke  [.] hashbrown::map::HashMap<K,V,S,A>::insert
+   10,53%    10,53%  aoc-2023-rust-f  aoc-2023-rust-flupke  [.] aoc_2023_rust_flupke::day_16::MirrorMap::trace
```

I could probably find a faster hash function for `Ray`, but I'm happy with this
performance exploration and will stop here :)
