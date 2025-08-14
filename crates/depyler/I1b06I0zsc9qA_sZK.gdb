# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp7in3tq/I1b06I0zsc9qA_sZK.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
