# 删除排序链表中的重复元素 II

存在一个按升序排列的链表，给你这个链表的头节点 head ，请你删除链表中所有存在数字重复情况的节点，只保留原始链表中 **没有重复出现** 的数字。

返回同样按升序排列的结果链表。

来源：力扣（LeetCode）
链接：<https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list-ii>
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

![image](./img/example.jpg)

关键点: 怎样拿到出项相同节点值时候的前一个节点, 然后 `prev.next = prev.next.next`
