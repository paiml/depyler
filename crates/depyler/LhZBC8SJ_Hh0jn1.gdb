# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpfHBmA1/LhZBC8SJ_Hh0jn1.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
