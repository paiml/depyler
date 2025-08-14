# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpKqh7RI/ug881EFatCx5VCx6.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
