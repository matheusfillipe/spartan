TARGET=spartan
BUILDDIR=build
COBOLDIR=cobol
COBENTRY=entry
ENTRY=main.rs
RC=rustc
COBC=cobc

all: $(BUILDDIR)/lib${COBENTRY}.so $(BUILDDIR)/$(TARGET)

run: all
	@echo "Running main target..."
	@echo "--------------------------------------------"
	@LD_LIBRARY_PATH=$(BUILDDIR) ./$(BUILDDIR)/$(TARGET)

$(BUILDDIR)/$(TARGET): ${ENTRY} libcob.rs socket.rs
	@echo "Building $<..."
	mkdir -p $(BUILDDIR)
	$(RC) $< -o $@ -L$(CURDIR)/$(BUILDDIR) -l$(COBENTRY)

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

$(BUILDDIR)/lib${COBENTRY}.so: $(COBOLDIR)/${COBENTRY}.cob
	@echo "Building plugins..."
	# $(COBC) -g -d -fimplicit-init -m $$plugin -O -o $(BUILDDIR)/lib$$(basename $${plugin%.*}).so;
	for plugin in $(COBOLDIR)/*.cob; do \
		$(COBC) -fimplicit-init -m $$plugin -O -o $(BUILDDIR)/lib$$(basename $${plugin%.*}).so; \
	done

clean:
	rm -rf $(BUILDDIR)/*
	@echo "cleaned!"

