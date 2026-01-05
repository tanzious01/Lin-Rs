import numpy as np

# Create your 4x3 matrix A
A = np.array([[1, 2, 3], 
              [4, 5, 6], 
              [7, 8, 9], 
              [10, 11, 12]], dtype=float)

# Perform QR
Q, R = np.linalg.qr(A)

R = Q.T.dot(A)

print("Matrix R:")
print(R.round(4)) # Rounding for readability
