# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp1TBUVb/lt13CxO7ccv1_r7l0V7.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
