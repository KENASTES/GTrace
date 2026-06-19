using System.Collections.Generic;

namespace Front_End.Models
{
    public class Point2D
    {
        public double x { get; set; }
        public double y { get; set; }
    }

    public class PreviewData
    {
        public List<List<Point2D>> copper_polygons { get; set; }
        public List<List<Point2D>> toolpaths { get; set; }
    }
}