using System.Collections.Generic;
using BenchmarkDotNet.Attributes;
using BenchmarkDotNet.Running;

BenchmarkRunner.Run<ListSummer>();

public class ListSummer
{
    private readonly List<int> m_Numbers = new();

    [GlobalSetup]
    public void Setup()
    {
        string[] contents = File.ReadAllLines("../../../../../../../../generator/numbers.txt");
        foreach (var line in contents)
        {
            m_Numbers.Add(int.Parse(line.Trim()));
        }
    }

    [Benchmark]
    public int Sum_For()
    {
        int sum = 0;

        for (int i = 0; i < m_Numbers.Count; i++)
        {
            sum += m_Numbers[i];
        }

        return sum;
    }

    [Benchmark]
    public int Sum_ForEach()
    {
        int sum = 0;

        foreach (var num in m_Numbers)
        {
            sum += num;
        }

        return sum;
    }

    [Benchmark]
    public int Sum_Linq()
    {
        return m_Numbers.Aggregate(0, (a, b) => a + b);
    }
}
