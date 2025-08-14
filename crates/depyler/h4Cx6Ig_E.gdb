# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp9whxnh/h4Cx6Ig_E.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
