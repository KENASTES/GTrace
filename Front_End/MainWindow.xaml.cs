#nullable enable  // 🌟 บรรทัดนี้สำคัญมาก! ช่วยแก้ Error เรื่องเครื่องหมาย ? ทั้งหมด
using System;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using System.Windows.Media;
using System.Runtime.InteropServices;
using Microsoft.Win32;
using System.Text.Json;
using Front_End.Models;
using System.Collections.Generic;

namespace Front_End
{
    public partial class MainWindow : Window
    {
        [DllImport("core_engine.dll", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
        public static extern IntPtr process_gerber_to_gcode(string path_ptr, string output_path, int feed_rate, int laser_power, int mirror_x, double isolation_width_mm);

        [DllImport("core_engine.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void free_json_string(IntPtr ptr);

        private string selectedFilePath = "";
        private string selectedOutputPath = "";
        
        private Point lastMousePosition;
        private bool isPanning = false;
        private PreviewData? currentPreviewData = null;

        private class DrawingCanvasHost : FrameworkElement
        {
            public DrawingVisual Visual { get; } = new DrawingVisual();
            public DrawingCanvasHost() { AddVisualChild(Visual); AddLogicalChild(Visual); }
            protected override int VisualChildrenCount => 1;
            protected override Visual GetVisualChild(int index) => Visual;
        }

        private DrawingCanvasHost? copperHost;
        private DrawingCanvasHost? toolpathHost;
        private DrawingCanvasHost? gridHost;
        private double previewMinX;
        private double previewMaxX;
        private double previewMinY;
        private double previewMaxY;
        private bool hasPreviewBounds = false;

        public MainWindow()
        {
            InitializeComponent();
            GenerateButton.Click += GenerateButtonClick;

            gridHost = new DrawingCanvasHost();
            copperHost = new DrawingCanvasHost();
            toolpathHost = new DrawingCanvasHost();
            
            PreviewCanvas.Children.Add(gridHost);
            PreviewCanvas.Children.Add(copperHost);
            PreviewCanvas.Children.Add(toolpathHost);
        }

        private void LogToConsole(string message)
        {
            ConsoleLog.AppendText($"\n> {message}");
            ConsoleLog.ScrollToEnd();
        }

        private void GenerateButtonClick(object sender, RoutedEventArgs e)
        {
            if (string.IsNullOrEmpty(selectedFilePath))
            {
                LogToConsole("ERROR: No file selected to convert.");
                return;
            }

            if (string.IsNullOrEmpty(selectedOutputPath))
            {
                LogToConsole("ERROR: No output path selected.");
                return;
            }

            if (!int.TryParse(FeedRateInput.Text, out int feedRate) || feedRate <= 0)
            {
                LogToConsole("ERROR: Feed Rate must be a positive number.");
                return;
            }

            if (!double.TryParse(IsolationWidthInput.Text, out double isoWidth) || isoWidth <= 0)
            {
                LogToConsole("ERROR: Border Width must be a positive number in millimeters.");
                return;
            }

            int laserPower = 215;
            int mirrorX = chkMirrorX.IsChecked == true ? 1 : 0;

            LogToConsole("----------------------------------");
            LogToConsole("Processing Gerber & Fetching Geometry for Preview...");
            LogToConsole($"Settings - Feed Rate: {feedRate} mm/min, Border Width: {isoWidth:0.###} mm, Mirror X: {mirrorX}");

            IntPtr jsonPtr = IntPtr.Zero;
            try
            {
                jsonPtr = process_gerber_to_gcode(selectedFilePath, selectedOutputPath, feedRate, laserPower, mirrorX, isoWidth);

                if (jsonPtr != IntPtr.Zero)
                {
                    // 🌟 ถ้าเคนใช้ .NET Framework รุ่นเก่าแล้ว Error ตรงนี้ ให้เปลี่ยนเป็น Marshal.PtrToStringAnsi(jsonPtr)
                    string jsonResult = Marshal.PtrToStringUTF8(jsonPtr) ?? "{}";
                    
                    free_json_string(jsonPtr);
                    jsonPtr = IntPtr.Zero;

                    currentPreviewData = JsonSerializer.Deserialize<PreviewData>(jsonResult);
                    
                    if (currentPreviewData != null)
                    {
                        LogToConsole("SUCCESS: Data received. Rendering Preview Content...");
                        RenderPcbPreview();
                        AutoFitView();
                    }
                }
                else
                {
                    LogToConsole("ERROR: Core Engine returned null pointer.");
                }
            }
            catch (Exception ex)
            {
                LogToConsole($"EXCEPTION: Error parsing output JSON -> {ex.Message}");
            }
            finally
            {
                if (jsonPtr != IntPtr.Zero) free_json_string(jsonPtr);
            }
        }

        private void RenderPcbPreview()
        {
            if (currentPreviewData == null || copperHost == null || toolpathHost == null) return;

            double minX = double.MaxValue, maxX = double.MinValue;
            double minY = double.MaxValue, maxY = double.MinValue;

            using (DrawingContext dc = copperHost.Visual.RenderOpen())
            {
                Brush copperBrush = new SolidColorBrush(Color.FromRgb(24, 75, 41)); 
                Pen copperPen = new Pen(copperBrush, 0.02);

                GeometryGroup group = new GeometryGroup();
                group.FillRule = FillRule.EvenOdd;

                foreach (var poly in currentPreviewData.copper_polygons)
                {
                    if (poly.Count < 2) continue;
                    
                    // 🌟 แก้ลอจิก Bounding Box ให้เก็บค่าพิกัดแรกด้วย
                    minX = Math.Min(minX, poly[0].x); maxX = Math.Max(maxX, poly[0].x);
                    minY = Math.Min(minY, poly[0].y); maxY = Math.Max(maxY, poly[0].y);

                    PathFigure figure = new PathFigure { StartPoint = new Point(poly[0].x, poly[0].y), IsClosed = true };
                    for (int i = 1; i < poly.Count; i++)
                    {
                        figure.Segments.Add(new LineSegment(new Point(poly[i].x, poly[i].y), true));
                    
                        minX = Math.Min(minX, poly[i].x); maxX = Math.Max(maxX, poly[i].x);
                        minY = Math.Min(minY, poly[i].y); maxY = Math.Max(maxY, poly[i].y);
                    }
                    
                    PathGeometry pathGeo = new PathGeometry();
                    pathGeo.Figures.Add(figure);
                    group.Children.Add(pathGeo);
                }
                dc.DrawGeometry(copperBrush, copperPen, group);
            }

            using (DrawingContext dc = toolpathHost.Visual.RenderOpen())
            {
                Pen toolpathPen = new Pen(new SolidColorBrush(Color.FromRgb(0, 191, 255)), 0.15); 
                toolpathPen.StartLineCap = PenLineCap.Round;
                toolpathPen.EndLineCap = PenLineCap.Round;

                foreach (var path in currentPreviewData.toolpaths)
                {
                    if (path.Count < 2) continue;
                    for (int i = 0; i < path.Count - 1; i++)
                    {
                        dc.DrawLine(toolpathPen, new Point(path[i].x, path[i].y), new Point(path[i + 1].x, path[i + 1].y));
                    }
                }
            }

            if (minX != double.MaxValue)
            {
                previewMinX = minX;
                previewMaxX = maxX;
                previewMinY = minY;
                previewMaxY = maxY;
                hasPreviewBounds = true;

                double widthMm = maxX - minX;
                double heightMm = maxY - minY;
                TxtBoardBounds.Text = $"Bounds: {widthMm:F2} x {heightMm:F2} mm";
                
                RenderGrid(minX, maxX, minY, maxY);
            }
        }

        private void RenderGrid(double minX, double maxX, double minY, double maxY)
        {
            if (gridHost == null) return;

            using (DrawingContext dc = gridHost.Visual.RenderOpen())
            {
                Pen gridPen = new Pen(new SolidColorBrush(Color.FromRgb(45, 45, 45)), 0.02);
                Pen majorGridPen = new Pen(new SolidColorBrush(Color.FromRgb(70, 70, 70)), 0.04);

                int startX = (int)Math.Floor(minX) - 10;
                int endX = (int)Math.Ceiling(maxX) + 10;
                int startY = (int)Math.Floor(minY) - 10;
                int endY = (int)Math.Ceiling(maxY) + 10;

                for (int x = startX; x <= endX; x++)
                {
                    dc.DrawLine(x % 5 == 0 ? majorGridPen : gridPen, new Point(x, startY), new Point(x, endY));
                }
                for (int y = startY; y <= endY; y++)
                {
                    dc.DrawLine(y % 5 == 0 ? majorGridPen : gridPen, new Point(startX, y), new Point(endX, y));
                }
            }
        }

        private void Viewport_MouseWheel(object sender, MouseWheelEventArgs e)
        {
            Point mousePos = e.GetPosition(PreviewCanvas);
            double zoomFactor = e.Delta > 0 ? 1.2 : 1 / 1.2;

            if (CanvasScale.ScaleX * zoomFactor > 0.5 && CanvasScale.ScaleX * zoomFactor < 200)
            {
                CanvasScale.ScaleX *= zoomFactor;
                CanvasScale.ScaleY *= zoomFactor; 
                CanvasTranslate.X = mousePos.X - (mousePos.X - CanvasTranslate.X) * zoomFactor;
                CanvasTranslate.Y = mousePos.Y - (mousePos.Y - CanvasTranslate.Y) * zoomFactor;
            }
        }

        private void Viewport_MouseDown(object sender, MouseButtonEventArgs e)
        {
            if (e.ChangedButton == MouseButton.Left || e.ChangedButton == MouseButton.Middle)
            {
                isPanning = true;
                lastMousePosition = e.GetPosition(ViewportContainer);
                ViewportContainer.CaptureMouse();
            }
        }

        private void Viewport_MouseMove(object sender, MouseEventArgs e)
        {
            Point currentPos = e.GetPosition(ViewportContainer);
            
            if (isPanning)
            {
                double deltaX = currentPos.X - lastMousePosition.X;
                double deltaY = currentPos.Y - lastMousePosition.Y;

                CanvasTranslate.X += deltaX;
                CanvasTranslate.Y += deltaY;
                lastMousePosition = currentPos;
            }

            Point canvasPos = e.GetPosition(PreviewCanvas);
            TxtCursorPos.Text = $"X: {canvasPos.X:F3}, Y: {canvasPos.Y:F3} mm";
        }

        private void Viewport_MouseUp(object sender, MouseButtonEventArgs e)
        {
            if (isPanning)
            {
                isPanning = false;
                ViewportContainer.ReleaseMouseCapture();
            }
        }

        private void AutoFitView()
        {
            if (!hasPreviewBounds || ViewportContainer.ActualWidth <= 0 || ViewportContainer.ActualHeight <= 0)
            {
                CanvasScale.ScaleX = 8;
                CanvasScale.ScaleY = -8;
                CanvasTranslate.X = 150;
                CanvasTranslate.Y = 350;
                return;
            }

            double boardWidth = Math.Max(previewMaxX - previewMinX, 0.001);
            double boardHeight = Math.Max(previewMaxY - previewMinY, 0.001);
            double padding = 40;
            double scaleX = Math.Max((ViewportContainer.ActualWidth - padding * 2) / boardWidth, 0.1);
            double scaleY = Math.Max((ViewportContainer.ActualHeight - padding * 2) / boardHeight, 0.1);
            double scale = Math.Min(scaleX, scaleY);

            CanvasScale.ScaleX = scale;
            CanvasScale.ScaleY = -scale;
            CanvasTranslate.X = (ViewportContainer.ActualWidth - boardWidth * scale) / 2 - previewMinX * scale;
            CanvasTranslate.Y = (ViewportContainer.ActualHeight + boardHeight * scale) / 2 + previewMinY * scale;
        }

        private void BtnFitToView_Click(object sender, RoutedEventArgs e) => AutoFitView();
        
        private void SelectFileButtonClick(object sender, RoutedEventArgs e)
        {
            OpenFileDialog openFileDialog = new OpenFileDialog { Filter = "Gerber files (*.GBL;*.GBR)|*.GBL;*.GBR|All files (*.*)|*.*" };
            if (openFileDialog.ShowDialog() == true)
            {
                selectedFilePath = openFileDialog.FileName;
                SelectedFileText.Text = selectedFilePath;
                LogToConsole($"File selected: {selectedFilePath}");
            }
        }

        private void SaveFileButtonClick(object sender, RoutedEventArgs e)
        {
            SaveFileDialog saveFileDialog = new SaveFileDialog { Filter = "G-Code files (*.gcode)|*.gcode|All files (*.*)|*.*" };
            if (saveFileDialog.ShowDialog() == true)
            {
                selectedOutputPath = saveFileDialog.FileName;
                OutputFilePathText.Text = selectedOutputPath;
                LogToConsole($"Output file path set: {System.IO.Path.GetFileName(selectedOutputPath)}");
            }
        }
    }
}