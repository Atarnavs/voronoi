package com.artem.voronoi_gui;

import javax.swing.*;
import java.awt.*;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.util.ArrayList;

public class Area extends JFrame {
    private final JPanel drawing_panel = new JPanel()
    {
        @Override
        public void paint(Graphics g){
            super.paint(g);
            setPoints(g);
            drawLines(g);
        }
    }
    ;
    private final JPanel infoPanel = new JPanel();
    private final JButton nextPointButton = new JButton();
    private ArrayList<Integer[]> pointList;
    private ArrayList<Integer[]> lineList = new ArrayList<>();
    private static final int SIZE = 1000;
    private static final int INFO_WIDTH = 40;
    private static final Connector connector = new Connector();
    public Area(ArrayList<Integer[]> pointList) {
        super("Voronoi diagram");
        this.pointList = pointList;
        setLayout(new BorderLayout());
        infoPanel.setLayout(new BorderLayout());
        add(drawing_panel, BorderLayout.CENTER);
        add(infoPanel, BorderLayout.EAST);
        nextPointButton.addActionListener(new ButtonListener());
        infoPanel.add(nextPointButton, BorderLayout.NORTH);
        setSize(SIZE + 15 + INFO_WIDTH, SIZE + 40);
        infoPanel.setSize(INFO_WIDTH, SIZE);
        drawing_panel.setSize(SIZE, SIZE);
        setDefaultCloseOperation(EXIT_ON_CLOSE);
        setLocationRelativeTo(null);
        Info info = connector.getLists();
        setPointList(info.point_list);
        setLineList(info.line_list);
        drawing_panel.setVisible(true);
        setResizable(false);
        setVisible(true);
    }
    private void setPoints(Graphics g) {
        for (Integer[] point: pointList) {
            int x = point[0]; // x-coordinate of the center
            int y = SIZE - point[1]; // y-coordinate of the center
            int radius = 3; // radius of the circle
            // Draw the point
            g.fillOval(x - radius, y - radius, 2 * radius, 2 * radius);
        }
    }
    private void drawLines(Graphics g) {
        for (Integer[] line: lineList) {
            g.drawLine(line[0],SIZE - line[1],line[2],SIZE - line[3]);
        }
    }
    public void addPoint(int x, int y) {
        this.pointList.add(new Integer[]{x, y});
        drawing_panel.repaint();
    }

    public void setPointList(ArrayList<Integer[]> pointList) {
        this.pointList = pointList;
        drawing_panel.repaint();
    }
    public void addLine(int x1, int y1, int x2, int y2) {
        lineList.add(new Integer[]{x1, y1, x2, y2});
        drawing_panel.repaint();
    }

    public void setLineList(ArrayList<Integer[]> lineList) {
        this.lineList = lineList;
    }

    private class ButtonListener implements ActionListener {
        @Override
        public void actionPerformed(ActionEvent e) {
            if (e == null) {
                return;
            }
            Info info = connector.getLists();
            setPointList(info.point_list);
            setLineList(info.line_list);
        }
    }
}
