using System;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using System.Runtime.InteropServices;
using Microsoft.Win32;

namespace Front_End
{
    public partial class MainWindow : Window
    {
        [DllImport("core_engine.dll", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
        public static extern int process_gerber_to_gcode(string path_ptr, string output_path, int feed_rate, int laser_power, int mirror_x);

        private string selectedFilePath = "";
        private string selectedOutputPath = "";

        public MainWindow()
        {
            InitializeComponent();
            
            GenerateButton.Click += GenerateButtonClick;
        }

        private void LogToConsole(string message)
        {
            ConsoleLog.AppendText($"\n> {message}");
            ConsoleLog.ScrollToEnd();
        }

        private void SelectFileButtonClick(object sender, RoutedEventArgs e)
        {
            OpenFileDialog openFileDialog = new OpenFileDialog();
            openFileDialog.Filter = "All files (*.*)|*.*";

            if (openFileDialog.ShowDialog() == true)
            {
                selectedFilePath = openFileDialog.FileName;
                
                SelectedFileText.Text = selectedFilePath;
                LogToConsole($"File selected: {selectedFilePath}");
            }
        }

        private void SaveFileButtonClick(object sender, RoutedEventArgs e)
        {
            SaveFileDialog saveFileDialog = new SaveFileDialog();
            saveFileDialog.Filter = "G-Code files (*.gcode)|*.gcode|All files (*.*)|*.*";

            if (!string.IsNullOrEmpty(SelectedFileText.Text) && SelectedFileText.Text != "No file selected")
            {
                saveFileDialog.FileName = System.IO.Path.GetFileNameWithoutExtension(selectedOutputPath);
            }

            if (saveFileDialog.ShowDialog() == true)
            {
                selectedOutputPath = saveFileDialog.FileName;
                OutputFilePathText.Text = selectedOutputPath;
                LogToConsole($"Output file path set: {System.IO.Path.GetFileName(selectedOutputPath)}");
            }
        }

        const int FIXED_LASER_POWER = 215;

        private void GenerateButtonClick(object sender, RoutedEventArgs e)
        {
            if (string.IsNullOrEmpty(selectedFilePath))
            {
                LogToConsole("ERROR: No file selected to converrt. Please select a Gerber file first.");
                return;
            }

            int feedRate = int.Parse(FeedRateInput.Text);
            int laserPower = FIXED_LASER_POWER;
            int mirrorX = chkMirrorX.IsChecked == true ? 1 : 0;

            LogToConsole("----------------------------------");
            LogToConsole("Starting G-Code generation...");
            LogToConsole($"Settings - Feed Rate: {feedRate} mm/min, Laser Power: {laserPower}, Mirror X: {mirrorX}");

            try
            {
                int result = process_gerber_to_gcode(selectedFilePath, selectedOutputPath, feedRate, laserPower, mirrorX);

                if (result == 1)
                {
                    LogToConsole("SUCCESS: Gerber processed successfully.");
                }
                else
                {
                    LogToConsole($"ERROR: Core Engine returned code {result}");
                }
            }
            catch (Exception ex)
            {
                LogToConsole($"EXCEPTION: DLL Error while processing Error code -> {ex.Message}");
            }
        }
    }
}