from z3 import Optimize, BitVec, sat, set_param

set_param(verbose=2)

opt = Optimize()

output = [2, 4, 1, 5, 7, 5, 0, 3, 4, 0, 1, 6, 5, 5, 3, 0]

solution = BitVec("solution", 64)

# In the beginning A should have the value of the solution
A = solution

# B and C are 0 in the input
B = 0
C = 0

# Reverse-engineered hash function
# The loop modifies the values of A, B and C after each iteration
for num in output:
    B = A % 8
    B = B ^ 5
    C = A / (1 << B)
    A = A / (1 << 3)
    B = B ^ C
    B = B ^ 6
    # After all those operations, B % 8 should be the corresponding digit in the
    # output
    opt.add(num == (B % 8))

# In the end A should be 0
opt.add(A == 0)

# Find smallest possible solution (there exist multiple solutions)
opt.minimize(solution)

if opt.check() == sat:
    print(opt.model().eval(solution))
else:
    print("No solutions")
