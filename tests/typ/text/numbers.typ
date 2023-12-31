// Test how numbers are displayed.

---
// Test numbers in text mode.
12 \
12.0 \
3.14 \
1234567890 \
0123456789 \
0 \
0.0 \
+0 \
+0.0 \
-0 \
-0.0 \
-1 \
-3.14 \
-9876543210 \
-0987654321 \
٣٫١٤ \
-٣٫١٤ \
-¾ \
#text(fractions: true)[-3/2] \
2022 - 2023 \
2022 -- 2023 \
2022--2023 \
2022-2023 \
٢٠٢٢ - ٢٠٢٣ \
٢٠٢٢ -- ٢٠٢٣ \
٢٠٢٢--٢٠٢٣ \
٢٠٢٢-٢٠٢٣ \
-500 -- -400

---
// Test integers.
#12 \
#1234567890 \
#0123456789 \
#0 \
#(-0) \
#(-1) \
#(-9876543210) \
#(-0987654321) \
#(4 - 8)

---
// Test floats.
#12.0 \
#3.14 \
#1234567890.0 \
#0123456789.0 \
#0.0 \
#(-0.0) \
#(-1.0) \
#(-9876543210.0) \
#(-0987654321.0) \
#(-3.14) \
#(4.0 - 8.0)

---
// Test the `str` function with integers.
#str(12) \
#str(1234567890) \
#str(0123456789) \
#str(0) \
#str(-0) \
#str(-1) \
#str(-9876543210) \
#str(-0987654321) \
#str(4 - 8)

---
// Test the `str` function with floats.
#str(12.0) \
#str(3.14) \
#str(1234567890.0) \
#str(0123456789.0) \
#str(0.0) \
#str(-0.0) \
#str(-1.0) \
#str(-9876543210.0) \
#str(-0987654321.0) \
#str(-3.14) \
#str(4.0 - 8.0)

---
// Test the `repr` function with integers.
#repr(12) \
#repr(1234567890) \
#repr(0123456789) \
#repr(0) \
#repr(-0) \
#repr(-1) \
#repr(-9876543210) \
#repr(-0987654321) \
#repr(4 - 8)

---
// Test the `repr` function with floats.
#repr(12.0) \
#repr(3.14) \
#repr(1234567890.0) \
#repr(0123456789.0) \
#repr(0.0) \
#repr(-0.0) \
#repr(-1.0) \
#repr(-9876543210.0) \
#repr(-0987654321.0) \
#repr(-3.14) \
#repr(4.0 - 8.0)
