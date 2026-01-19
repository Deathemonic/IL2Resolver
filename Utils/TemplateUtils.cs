namespace IL2Resolver.Utils;

public static class TemplateUtils
{
    public static List<string> SplitTemplateArgs(string args)
    {
        var result = new List<string>();
        var depth = 0;
        var start = 0;

        for (var i = 0; i < args.Length; i++)
        {
            switch (args[i])
            {
                case '<':
                    depth++;
                    break;
                case '>':
                    depth--;
                    break;
                case ',' when depth == 0:
                    result.Add(args[start..i].Trim());
                    start = i + 1;
                    break;
            }
        }

        if (start < args.Length)
            result.Add(args[start..].Trim());

        return result;
    }
}
