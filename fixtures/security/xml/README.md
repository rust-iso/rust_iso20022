# Minimized hostile XML corpus

These synthetic fixtures contain no real financial data. They pin forbidden
DTD/entity/XInclude/non-UTF-8 declarations, malformed namespace/structure, deep
nesting, and large-collection shapes. Tests apply deliberately small limits to
the structural templates so the tracked corpus stays small while exercising
the same bounded-reader failure paths as oversized production inputs.
