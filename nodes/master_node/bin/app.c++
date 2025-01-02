import master_node;
import point_3D;
#include <iostream>
#include <vector>

int main()
{
	MasterNode master("192.168.0.100", "9090");

	master.allocate_worker_nodes();
	master.allocate_worker_nodes();
	master.allocate_worker_nodes();
	
}