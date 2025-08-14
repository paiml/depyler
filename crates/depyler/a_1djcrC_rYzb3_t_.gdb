# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpety4ZZ/a_1djcrC_rYzb3_t_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
