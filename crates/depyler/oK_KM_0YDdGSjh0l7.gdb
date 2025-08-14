# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp6P2ljJ/oK_KM_0YDdGSjh0l7.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
