# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp9Q6hVs/b8CeL2xuLEawJE0aLtfq2.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
