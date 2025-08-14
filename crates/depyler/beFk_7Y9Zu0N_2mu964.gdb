# GDB initialization script for Depyler debugging
# Source: /tmp/.tmpfUYflV/beFk_7Y9Zu0N_2mu964.py

directory .

# Load Rust pretty printers
python
import gdb
gdb.execute('set print pretty on')
end
