# GDB initialization script for Depyler debugging
# Source: /tmp/.tmp0XtTpq/n__kElO_b__gCW36z3u3v.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
