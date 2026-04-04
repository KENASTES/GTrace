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
        public static extern int process_gerber_to_gcode(string path_ptr);

        private string selectedFilePath = "";

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
            openFileDialog.Filter = "Gerber Files (*.grb;*.gbr)|*.grb;*.gbr|All files (*.*)|*.*";

            if (openFileDialog.ShowDialog() == true)
            {
                selectedFilePath = openFileDialog.FileName;
                
                SelectedFileText.Text = selectedFilePath;
                LogToConsole($"File selected: {selectedFilePath}");
            }
        }

        private void GenerateButtonClick(object sender, RoutedEventArgs e)
        {
            if (string.IsNullOrEmpty(selectedFilePath))
            {
                MessageBox.Show("กรุณาเลือกไฟล์ Gerber ก่อนครับ!", "Warning", MessageBoxButton.OK, MessageBoxImage.Warning);
                return;
            }

            string feedRate = FeedRateInput.Text;
            string laserPower = LaserPowerInput.Text; 

            LogToConsole("----------------------------------");
            LogToConsole("Starting G-Code generation...");
            LogToConsole($"Settings - Feed Rate: {feedRate} mm/min, Laser Power: {laserPower}");

            try
            {
                int result = process_gerber_to_gcode(selectedFilePath);

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
                LogToConsole($"EXCEPTION: DLL ไม่พร้อมทำงาน หรือเกิดข้อผิดพลาด -> {ex.Message}");
            }
        }
    }
}