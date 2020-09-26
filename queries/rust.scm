(use_declaration
	argument: (scoped_identifier name: (identifier) @definition.imported))

(function_item
 body: (block (let_declaration
		 pattern: (identifier) @definition.scoped)))

(function_item
	(visibility_modifier)
    name: (identifier) @definition.exported)
