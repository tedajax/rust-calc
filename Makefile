TARGET		= calc
LIBDIR		= lib
SRCDIR		= src
SOURCES		= main.rs
rm		= rm -rf

all:
	rustc -L $(LIBDIR) $(SRCDIR)/$(SOURCES) -o $(TARGET) -g

run: all
	./$(TARGET)

clean:
	@$(rm) $(TARGET) $(TARGET).dSYM