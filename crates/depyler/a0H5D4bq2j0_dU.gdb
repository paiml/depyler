# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp84mse4/a0H5D4bq2j0_dU.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
