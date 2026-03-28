# Chaikin Algorithm Explanation

## How the Math Works

Let me visualize how the Chaikin algorithm works step by step:

```
ORIGINAL LINE SEGMENT:
======================

Start Point (A)                                    End Point (B)
   (100, 200)                                         (300, 200)
       •━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━•
       |<-------------- 200 pixels wide ------------->|


FINDING THE QUARTER POINT (Q):
================================

Quarter Point Formula:  Q = A * 0.75 + B * 0.25

   A * 0.75                    +          B * 0.25
(100, 200) * 0.75                     (300, 200) * 0.25
    = (75, 150)                           = (75, 50)

                         ADD THEM TOGETHER

                    Q = (75, 150) + (75, 50)
                    Q = (150, 200)


Start (A)          Quarter (Q)                        End (B)
 (100,200)         (150,200)                         (300,200)
    •━━━━━━━━━━━━━━━•━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━•
    |<-- 50 px -->|<---------- 150 pixels ---------->|

    75% of A's      25% of B's
    contribution    contribution


FINDING THE THREE-QUARTER POINT (R):
======================================

Three-Quarter Point Formula:  R = A * 0.25 + B * 0.75

   A * 0.25                    +          B * 0.75
(100, 200) * 0.25                     (300, 200) * 0.75
    = (25, 50)                           = (225, 150)

                         ADD THEM TOGETHER

                    R = (25, 50) + (225, 150)
                    R = (250, 200)


Start (A)          Quarter (Q)    3-Quarter (R)      End (B)
 (100,200)         (150,200)       (250,200)        (300,200)
    •━━━━━━━━━━━━━━━•━━━━━━━━━━━━━━━•━━━━━━━━━━━━━━━•
    |<-- 50 px -->|<--- 100 px --->|<--- 50 px --->|


CHAIKIN RESULT - REPLACE 1 SEGMENT WITH 2 NEW POINTS:
======================================================

BEFORE:
A━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━B

AFTER:
A     Q           R     B
•━━━━━•━━━━━━━━━━━•━━━━━•
      (new points replace the line, corners get smoothed!)


EXAMPLE WITH MULTIPLE SEGMENTS:
================================

Original Triangle (3 points):
        P1
        •
       /  \
      /    \
     /      \
    •────────•
   P0        P2

After Chaikin Step (adds Q and R for each segment):
        P1

      Q₁  R₁
     •      •
    /        \
  R₀          Q₂
  •            •
   \          /
    •────────•
    Q₀      R₂

Notice: The sharp corners are "cut off" and replaced with smoother curves!
```

## Key Insight

The multiplication creates a **weighted average**:

- `0.75` means "mostly the first point" → point closer to start
- `0.25` means "mostly the second point" → point closer to end

When you add up `0.75 + 0.25 = 1.0`, you get a point **exactly on the line** between the two original points, just positioned at the right spot!

Each iteration "cuts the corners" of your shape, making it progressively smoother. 🎨

## Linear Interpolation (Lerp)

We're using **linear interpolation** to find points between two positions.

**The formula:**

```rust
let quarter_point = start_point * 0.75 + end_point * 0.25;
```

This creates a **weighted average** to find a point between two locations.

### Why it works:

- When we multiply by 0.75 and 0.25, the weights **add up to 1.0**
- This ensures the result stays on the line between the two points
- The ratio (0.75:0.25) determines **how far** along the line we go

### Visual representation:

```
start (100%)     quarter (75%)     three-quarter (25%)     end (0%)
    •----------------•----------------------•-----------------•
  100,200         125,250                175,350          200,400
```

It's basically saying: "Give me 75% of the start position plus 25% of the end position" to get a point 1/4 of the way along the line!
