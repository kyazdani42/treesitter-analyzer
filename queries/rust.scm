(use_declaration
	argument: (scoped_identifier name: (identifier) @definition.imported))

(use_declaration
	argument: (scoped_use_list list: (use_list (identifier) @definition.imported)))

(let_declaration
		 pattern: (identifier) @definition.scoped)

(function_item
	(visibility_modifier)
    name: (identifier) @definition.exported)

(function_item
    name: (identifier) @definition.scoped)

(struct_item
	(visibility_modifier)
	name: (type_identifier) @definition.exported)

(struct_item
	name: (type_identifier) @definition.scoped)


