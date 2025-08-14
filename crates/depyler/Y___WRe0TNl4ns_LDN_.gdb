# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp9hLtZl/Y___WRe0TNl4ns_LDN_.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
