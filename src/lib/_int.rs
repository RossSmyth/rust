import std.sys;

fn add(int x, int y) -> int { ret x + y; }
fn sub(int x, int y) -> int { ret x - y; }
fn mul(int x, int y) -> int { ret x * y; }
fn div(int x, int y) -> int { ret x / y; }
fn rem(int x, int y) -> int { ret x % y; }

fn lt(int x, int y) -> bool { ret x < y; }
fn le(int x, int y) -> bool { ret x <= y; }
fn eq(int x, int y) -> bool { ret x == y; }
fn ne(int x, int y) -> bool { ret x != y; }
fn ge(int x, int y) -> bool { ret x >= y; }
fn gt(int x, int y) -> bool { ret x > y; }

iter range(mutable int lo, int hi) -> int {
  while (lo < hi) {
    put lo;
    lo += 1;
  }
}

iter urange(mutable uint lo, uint hi) -> uint {
  while (lo < hi) {
    put lo;
    lo += 1u;
  }
}

fn next_power_of_two(uint n) -> uint {
  // FIXME change |* uint(4)| below to |* uint(8) / uint(2)| and watch the
  // world explode.
  let uint halfbits = sys.rustrt.size_of[uint]() * 4u;
  let uint tmp = n - 1u;
  let uint shift = 1u;
  while (shift <= halfbits) {
    tmp |= tmp >> shift;
    shift <<= 1u;
  }
  ret tmp + 1u;
}
