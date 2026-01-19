using System.Collections.Frozen;

namespace IL2Resolver.Utils;

public static class RustKeywords
{
    private static readonly FrozenDictionary<string, string> Renames = new Dictionary<string, string>
    {
        ["as"] = "as_value",
        ["break"] = "break_value",
        ["const"] = "const_value",
        ["continue"] = "continue_loop",
        ["crate"] = "crate_ref",
        ["else"] = "else_branch",
        ["enum"] = "enum_value",
        ["extern"] = "extern_ref",
        ["false"] = "false_value",
        ["fn"] = "callback",
        ["for"] = "for_target",
        ["if"] = "if_condition",
        ["impl"] = "impl_ref",
        ["in"] = "in_value",
        ["let"] = "let_value",
        ["loop"] = "loop_ref",
        ["match"] = "match_value",
        ["mod"] = "mod_ref",
        ["move"] = "move_value",
        ["mut"] = "mut_ref",
        ["pub"] = "pub_value",
        ["ref"] = "ref_value",
        ["return"] = "return_value",
        ["self"] = "this",
        ["Self"] = "this_type",
        ["static"] = "static_ref",
        ["struct"] = "struct_ref",
        ["super"] = "super_ref",
        ["trait"] = "trait_ref",
        ["true"] = "true_value",
        ["type"] = "type_ref",
        ["unsafe"] = "unsafe_ref",
        ["use"] = "use_ref",
        ["where"] = "where_clause",
        ["while"] = "while_loop",
        ["async"] = "async_ref",
        ["await"] = "await_ref",
        ["dyn"] = "dyn_ref",
        ["abstract"] = "abstract_ref",
        ["become"] = "become_ref",
        ["box"] = "draw_box",
        ["do"] = "do_action",
        ["final"] = "final_value",
        ["macro"] = "macro_ref",
        ["override"] = "override_ref",
        ["priv"] = "priv_ref",
        ["typeof"] = "typeof_ref",
        ["unsized"] = "unsized_ref",
        ["virtual"] = "virtual_ref",
        ["yield"] = "yield_value",
        ["try"] = "try_result",
    }.ToFrozenDictionary();

    public static string Escape(string name) => Renames.GetValueOrDefault(name, name);
}
